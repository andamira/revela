// revela::backend::midir
//
//! `midir` backend.
//
// - https://docs.rs/midir/latest/midir/index.html
//
// ISSUES:
// - [ ] Why closures, why threads? https://github.com/Boddlnagg/midir/issues/118
// - [ ] Jack is non "realtime safe": https://github.com/Boddlnagg/midir/issues/102

/// Re-export of the [`midir`](https://docs.rs/midir) crate.
#[doc(inline)]
pub use ::midir;

use midir::{
    MidiInput as MidirInput, MidiInputConnection as MidirInputConnection,
    MidiInputPort as MidirInputPort, MidiOutput as MidirOutput,
    MidiOutputConnection as MidirOutputConnection, MidiOutputPort as MidirOutputPort,
};
// pub use midir::Ignore as MidirIgnore; // MAYBE make own type, impl From & Default::None
use flume::{self, Receiver, Sender, TryRecvError};
use midi_convert::{midi_types::MidiMessage, MidiTryParseSlice};

use crate::all::{
    Backend, Capabilities, Event, EventSource, InputCapabilities, MidiEvent, RevelaError as Error,
    RevelaResult as Result,
};

use std::collections::HashMap;

/// `midir` interface.
pub struct MidirBackend {
    /// unconnected input port, used for querying.
    input: MidirInput,
    /// list of connected input ports.
    input_connections: HashMap<String, MidirInputConnection<()>>,

    /// unconnected output port, used for querying.
    output: MidirOutput,
    /// list of connected output ports.
    output_connections: HashMap<String, MidirOutputConnection>,

    input_producer: Sender<(MidiEvent, u64)>,
    input_consumer: Receiver<(MidiEvent, u64)>,
    // MAYBE
    // ignored: MidirIgnore,
}

impl Backend for MidirBackend {
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            input: Some(InputCapabilities {
                midi: true,
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    fn version(&self) -> (u32, u32, u32) {
        // IMPROVE:detect-version
        (0, 9, 1)
    }
}

impl EventSource for MidirBackend {
    fn wait_event(&mut self) -> Result<Event> {
        // TODO
        // self.input_consumer.recv()
        Err(Error::NotSupported)
    }
    // https://docs.rs/flume/latest/flume/struct.Receiver.html
    fn poll_event(&mut self) -> Result<Event> {
        match self.input_consumer.try_recv() {
            Ok((event, µs)) => Ok(Event::new(event.into(), Some(µs.into()))),
            Err(e) => match e {
                TryRecvError::Empty => Ok(Event::None),
                TryRecvError::Disconnected => Err(e.into()),
            },
        }
    }
}

impl MidirBackend {
    /// Returns a new midir gamepad event source, with default settings
    pub fn new() -> Result<Self> {
        let input = MidirInput::new("midir default input")?;
        let input_connections = HashMap::new();
        let output = MidirOutput::new("midir default ouput")?;
        let output_connections = HashMap::new();

        // MAYBE
        // let ignored = MidirIgnore::None;

        let (input_producer, input_consumer) = flume::unbounded();

        Ok(Self {
            input,
            input_connections,
            input_producer,
            input_consumer,
            output,
            output_connections,
            // output_producer,
            // output_consumer,
            // ignored,
        })
    }

    /// Returns a vector of midi input ports.
    pub fn in_ports(&self) -> Vec<MidirInputPort> {
        self.input.ports()
    }
    /// Returns the count of midi input ports.
    pub fn in_count(&self) -> usize {
        self.input.port_count()
    }
    /// Returns a vector with the names of the midi input ports.
    pub fn in_names(&self) -> Vec<String> {
        let in_ports = self.in_ports();
        let mut in_names = Vec::with_capacity(in_ports.len());
        for p in in_ports {
            let in_name = self.input.port_name(&p).expect("invalid port");
            in_names.push(in_name);
        }
        in_names
    }
    /// Returns the number of active input connections.
    pub fn in_connections(&self) -> usize {
        self.input_connections.len()
    }

    /// Connects to the requested input `port`, and associates the connection to
    /// the given `name`.
    //
    // https://docs.rs/midir/latest/midir/struct.MidiInput.html#method.connect
    pub fn in_connect(&mut self, port: &MidirInputPort, name: &str) -> Result<()> {
        let input = MidirInput::new(name)?;
        let producer = self.input_producer.clone();

        let connection = input.connect(
            port,
            name,
            move |µs: u64, bytes: &[u8], _data| {
                // log::trace!["producing event: {µs} {bytes:?} {_data:?}"];
                let kind = MidiMessage::try_parse_slice(bytes).expect("midi parse error");
                producer.send((kind, µs)).expect("send");
            },
            (),
        )?;

        self.input_connections.insert(name.into(), connection);
        Ok(())
    }
    /// Connects to the input port at the provided `index`.
    ///
    /// # Error
    /// If the index is out of bounds, or there's some connection error.
    pub fn in_connect_index(&mut self, index: usize, name: &str) -> Result<()> {
        if let Some(in_port) = self.in_ports().get(index) {
            self.in_connect(in_port, name)?;
            Ok(())
        } else {
            Err(Error::string("midi port index out of bounds."))
        }
    }
    /// Connects to all the input ports.
    pub fn in_connect_all(&mut self) -> Result<()> {
        for (num, in_port) in self.in_ports().iter().enumerate() {
            self.in_connect(in_port, &format!["in_port {num}"])?;
        }
        Ok(())
    }
    /// Connects to the last input port.
    ///
    /// # Error
    /// If the index is out of bounds, or there's some connection error.
    pub fn in_connect_last(&mut self, name: &str) -> Result<()> {
        if let Some(in_port) = self.in_ports().last() {
            self.in_connect(in_port, name)?;
            // } else {
            //     Err(Error::string("there are no available input midi ports."))
        }
        Ok(())
    }

    // TODO. NOTE the key is the name, not the input port.
    // /// Closes an active input connection.
    // pub fn in_close(&mut self, in_port: &MidirInputPort) -> Result<()> {
    //     todo![]
    // }

    /// Returns a vector of midi output ports.
    pub fn out_ports(&self) -> Vec<MidirOutputPort> {
        self.output.ports()
    }
    /// Returns the count of midi output ports.
    pub fn out_count(&self) -> usize {
        self.output.port_count()
    }
    /// Returns a vector with the names of the midi output ports.
    pub fn out_names(&self) -> Vec<String> {
        let out_ports = self.out_ports();
        let mut out_names = Vec::with_capacity(out_ports.len());
        for p in out_ports {
            let out_name = self.output.port_name(&p).expect("invalid port");
            out_names.push(out_name);
        }
        out_names
    }
    /// Returns the number of active output connections.
    pub fn out_connections(&self) -> usize {
        self.output_connections.len()
    }

    // MAYBE
    // // https://docs.rs/midir/latest/midir/enum.Ignore.html
    // pub fn ignore(&mut self) {
    //     todo![]
    //     // midi_in.ignore(MidirIgnore::None);
    // }
}

mod std_impls {
    use super::{Backend, MidirBackend};
    use std::fmt;

    impl fmt::Debug for MidirBackend {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "MidirBackend {{ {}, in:{}/{} out:{}/{} }}",
                self.version_string(),
                self.in_connections(),
                self.in_count(),
                self.out_connections(),
                self.out_count(),
                // self.ignored,
            )
        }
    }
}

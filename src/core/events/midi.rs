// revela::core::events::midi
//
//!
//

use super::Event;
use midi_convert::midi_types::MidiMessage;

pub use midi_convert::midi_types::{
    Channel as MidiChannel, Control as MidiControl, Note as MidiNote, Program as MidiProgram,
    QuarterFrame as MidiFrame, Value14 as MidiValue14, Value7 as MidiValue7,
};

///
// TEMP: until original derives Copy
// https://github.com/rust-midi/midi-types/pull/13
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MidiEvent {
    NoteOff(MidiChannel, MidiNote, MidiValue7),
    NoteOn(MidiChannel, MidiNote, MidiValue7),
    KeyPressure(MidiChannel, MidiNote, MidiValue7),
    ControlChange(MidiChannel, MidiControl, MidiValue7),
    ProgramChange(MidiChannel, MidiProgram),
    ChannelPressure(MidiChannel, MidiValue7),
    PitchBendChange(MidiChannel, MidiValue14),
    QuarterFrame(MidiFrame),
    SongPositionPointer(MidiValue14),
    SongSelect(MidiValue7),
    TuneRequest,
    TimingClock,
    Start,
    Continue,
    Stop,
    ActiveSensing,
    Reset,
}

impl From<MidiMessage> for MidiEvent {
    fn from(from: MidiMessage) -> MidiEvent {
        use MidiMessage::*;
        match from {
            NoteOff(ch, n, v7) => MidiEvent::NoteOff(ch, n, v7),
            NoteOn(ch, n, v7) => MidiEvent::NoteOn(ch, n, v7),
            KeyPressure(ch, n, v7) => MidiEvent::KeyPressure(ch, n, v7),
            ControlChange(ch, co, v7) => MidiEvent::ControlChange(ch, co, v7),
            ProgramChange(ch, p) => MidiEvent::ProgramChange(ch, p),
            ChannelPressure(ch, v7) => MidiEvent::ChannelPressure(ch, v7),
            PitchBendChange(ch, v14) => MidiEvent::PitchBendChange(ch, v14),
            QuarterFrame(f) => MidiEvent::QuarterFrame(f),
            SongPositionPointer(v14) => MidiEvent::SongPositionPointer(v14),
            SongSelect(v7) => MidiEvent::SongSelect(v7),
            TuneRequest => MidiEvent::TuneRequest,
            TimingClock => MidiEvent::TimingClock,
            Start => MidiEvent::Start,
            Continue => MidiEvent::Continue,
            Stop => MidiEvent::Stop,
            ActiveSensing => MidiEvent::ActiveSensing,
            Reset => MidiEvent::Reset,
        }
    }
}

impl From<MidiEvent> for Event {
    fn from(from: MidiEvent) -> Event {
        Event::Midi(from)
    }
}

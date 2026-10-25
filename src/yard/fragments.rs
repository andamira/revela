//
//! Docs headings for modules and misc. abbreviations.
//
// NOTES
// - `_DOC_*` defines the canonical opening sentence of a public module.
// - Parent-module declaration comments provide local navigation and may paraphrase it.
// - Explicit re-export lists describe the actual forwarded surface when appropriate.
// - Longer explanations remain in each module's source.

crate::CONST! { hidden macro_export,
    /* misc. root modules */

    _DOC_ALL  = "All crate items re-exported in a single flat namespace.\n\n
<br/>Every item in this namespace is forwarded upward, as every other public module does.";

    _DOC_ALL_ = "All crate items re-exported, grouped by their root modules.";
    _DOC_ALL_PLUS_ = "\n\nEach root module appears here and provides
its own flat view of all its public children.

Click on the `▽` symbol after the list of submodules to switch to hierarchical view,
and `◉` to switch back to flat view.
";

    /* thematic modules tree */

    // 1. engine (how something proceeds)
    _DOC_ENGINE  = "Machinery for operating, transforming, and coordinating modeled systems.";

    // 2. model (expression and understanding)
    _DOC_MODEL   = "Structures for representing, relating, and interpreting modeled data.";

    // 3. org (agency beyond the individual)
    _DOC_ORG            = "Coordination and structure of collective action.";
    _DOC_ORG_AGENT      = "Intentional entities capable of action and coordination.";
    _DOC_ORG_ECON       = "Exchange, incentives, and flows of value within collectives.";
    _DOC_ORG_GOV        = "Governance, authority, and rule-based coordination.";
    _DOC_ORG_INST       = "Enduring organizational forms and institutional structures.";
    _DOC_ORG_MORAL      = "Normative frameworks of responsibility, duty, and judgment.";
    _DOC_ORG_NORM       = "Shared social norms and informal behavioral expectations.";
    _DOC_ORG_ROLE       = "Roles, responsibilities, and positions within coordinated action.";

    // 4. phys (nature made measurable)
    _DOC_PHYS           = "Physical quantities, units, and models of the natural world.";
    _DOC_PHYS_ASTRO     = "Astronomy-related abstractions.";
    _DOC_PHYS_BIO       = "Biology-related abstractions.";
    _DOC_PHYS_CHEM      = "Chemistry-related abstractions.";
    _DOC_PHYS_ELEC      = "Electrical state, fields, transport, and energetic coupling.";
    _DOC_PHYS_GEO       = "Geophysics-related abstractions.";
    _DOC_PHYS_HEAT      = "Thermodynamics and heat transfer.";
    _DOC_PHYS_MECH      = "Mechanical state, response, interaction, and transmission.";
    _DOC_PHYS_OPTIC     = "Optics and light transport in piecewise media.";
    _DOC_PHYS_SUBS      = "Physical substrates, bodies, phases, and material response properties.";
    _DOC_PHYS_TIME      = "Time and calendar types and operations.";
    _DOC_PHYS_UNIT      = "Physical units of measure and unit prefixes.";
    _DOC_PHYS_WAVE      = "Wave primitives, wavelets.";

    // 5. vita (life as capacity and practice)
    _DOC_VITA           = "Lived practices of embodied beings.";
    _DOC_VITA_BODY      = "Embodied capability, health, and physical limits of living beings.";
    _DOC_VITA_CRAFT     = "Practical, learned ways of shaping material reality to support life.";
    _DOC_VITA_HOME      = "Inhabited space, from dwelling to built and shared environments.";
    _DOC_VITA_LOVE      = "Relational life, from kinship and care to intimacy and community.";
    _DOC_VITA_MIND      = "Cognition, memory, meaning, and inner orientation of lived experience.";
    _DOC_VITA_PLAY      = "Expression, play, and shared enjoyment beyond necessity or survival.";
    _DOC_VITA_PLAY_GAME = "Games across cultures.";

    // 6. yard (where the crate tends itself)
    _DOC_YARD           = "Scaffolding, taxonomy, and documentation support.";
}

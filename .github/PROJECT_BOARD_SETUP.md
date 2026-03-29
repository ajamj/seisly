# GitHub Project Board Setup Instructions

## Create Project Board

1. Navigate to: https://github.com/ajamj/StrataForge/projects
2. Click **"New project"**
3. Select **"Project board"** template (Kanban style)
4. Name: **"Phase 0 - v0.2.0 Foundation"**
5. Click **"Create"**

## Configure Columns

Delete default columns and add these:

| Column | Purpose |
|--------|---------|
| 📋 Backlog | All planned tasks |
| 🔄 In Progress | Currently being worked on |
| 👀 In Review | Ready for code review |
| ✅ Done | Completed and merged |

## Add Issues

Create the following issues (copy-paste templates below):

### Issue 1: FormationTop Domain Model
```markdown
## Task
Implement FormationTop domain model in sf_core crate.

## Acceptance Criteria
- [ ] FormationTop struct with id, well_id, name, depth_md fields
- [ ] Support for optional formation name and comments
- [ ] Serialization via serde (JSON, YAML)
- [ ] Unit tests for creation and serialization
- [ ] Module exported in sf_core::domain

## Files to Create
- `crates/sf_core/src/domain/formation_top.rs`

## Files to Modify
- `crates/sf_core/src/domain.rs` - Export new module

## Implementation Plan
See: `docs/superpowers/plans/2026-03-29-phase-0-foundation.md#task-1

## Estimated Time
2-3 hours

## Priority
🔴 High (blocks other tasks)

## Labels
- enhancement
- sf_core
- good first issue
```

### Issue 2: Complete SEG-Y Reader
```markdown
## Task
Implement complete SEG-Y reader with memory-mapped access.

## Acceptance Criteria
- [ ] SegyReader with open() method
- [ ] Textual (EBCDIC) header support
- [ ] Binary header parsing
- [ ] Trace-level read access
- [ ] Integration tests

## Files to Create
- `crates/sf_io/src/segy/reader.rs`

## Files to Modify
- `crates/sf_io/src/segy/mod.rs`
- `crates/sf_io/Cargo.toml` - Add segy-rs dependency

## Implementation Plan
See: `docs/superpowers/plans/2026-03-29-phase-0-foundation.md#task-2

## Estimated Time
4-6 hours

## Priority
🔴 High

## Labels
- enhancement
- sf_io
- segy
```

### Issue 3: Complete SEG-Y Writer
```markdown
## Task
Implement complete SEG-Y writer.

## Acceptance Criteria
- [ ] SegyWriter with configurable parameters
- [ ] Trace-by-trace writing API
- [ ] Automatic header generation
- [ ] Round-trip test with SegyReader

## Files to Create
- `crates/sf_io/src/segy/writer.rs`
- `crates/sf_io/tests/segy_writer_test.rs`

## Implementation Plan
See: `docs/superpowers/plans/2026-03-29-phase-0-foundation.md#task-3

## Estimated Time
3-4 hours

## Priority
🟡 Medium

## Labels
- enhancement
- sf_io
- segy
```

### Issue 4: LAS 3.0 Parser
```markdown
## Task
Add LAS 3.0 parser support.

## Acceptance Criteria
- [ ] LasV3Reader with enhanced metadata support
- [ ] JSON-like section parsing
- [ ] Backward compatible with LAS 2.0
- [ ] Unit tests

## Files to Create
- `crates/sf_io/src/las/v3.rs`

## Files to Modify
- `crates/sf_io/src/las/mod.rs`

## Implementation Plan
See: `docs/superpowers/plans/2026-03-29-phase-0-foundation.md#task-4

## Estimated Time
4-5 hours

## Priority
🔴 High

## Labels
- enhancement
- sf_io
- las
```

### Issue 5: Well-Seismic Tie Engine
```markdown
## Task
Implement well-seismic tie computation module.

## Acceptance Criteria
- [ ] WellTieEngine with replacement velocity method
- [ ] Time-depth pair generation
- [ ] Bidirectional depth<->TWT conversion
- [ ] Unit tests for accuracy

## Files to Create
- `crates/sf_compute/src/well_tie.rs`
- `crates/sf_compute/tests/well_tie_test.rs`

## Implementation Plan
See: `docs/superpowers/plans/2026-03-29-phase-0-foundation.md#task-5

## Estimated Time
3-4 hours

## Priority
🔴 High

## Labels
- enhancement
- sf_compute
- well-tie
```

### Issue 6: Documentation Update
```markdown
## Task
Update documentation for v0.2 features.

## Acceptance Criteria
- [ ] Well-seismic tie user guide
- [ ] README.md feature list updated
- [ ] API documentation for new modules
- [ ] Migration guide (if needed)

## Files to Create
- `docs/well_seismic_tie.md`

## Files to Modify
- `README.md`

## Implementation Plan
See: `docs/superpowers/plans/2026-03-29-phase-0-foundation.md#task-6

## Estimated Time
2-3 hours

## Priority
🟡 Medium

## Labels
- documentation
```

### Issue 7: Integration Tests
```markdown
## Task
Create comprehensive integration tests for Phase 0.

## Acceptance Criteria
- [ ] Full SEG-Y roundtrip test
- [ ] LAS 3.0 parsing test
- [ ] Well tie computation test
- [ ] All tests passing on CI

## Files to Create
- `crates/sf_io/tests/integration_test.rs`

## Implementation Plan
See: `docs/superpowers/plans/2026-03-29-phase-0-foundation.md#task-7

## Estimated Time
2-3 hours

## Priority
🟡 Medium

## Labels
- testing
- sf_io
- sf_compute
```

## Create Milestone

1. Go to: https://github.com/ajamj/StrataForge/milestones
2. Click **"New milestone"**
3. Name: **"v0.2.0 - Phase 0 Foundation"**
4. Description: "Complete well-seismic workflow: SEG-Y, LAS 3.0, well tie, formation tops"
5. Due date: **6 weeks from today**
6. Click **"Create milestone"**
7. Assign all issues above to this milestone

## Add Project Automations

Add these automations (Project settings → Automations):

- When issue is opened → Add to "📋 Backlog"
- When issue is closed → Move to "✅ Done"
- When PR is opened → Move to "👀 In Review"

## Final Check

Your project board should now have:
- ✅ 7 issues in "📋 Backlog" column
- ✅ All issues assigned to v0.2.0 milestone
- ✅ Appropriate labels on each issue
- ✅ Links to implementation plan in descriptions

---

**Next Step:** Share project board link with team and start sprint!

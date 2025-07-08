# French Vocabulary Review Tool

## Usage

### Dependencies

TODO

### After `git clone`

TODO

### To Run

To run frontend app server, `cd client` and run `npm run dev`.

To run backend Rust API server, run `cargo run`.

Access app via local port corresponding to frontend app server.

## Proposed Features

### Word/Phrase Storage

- Somewhat immutable SQLite database (other than database edit tool & quick population script)
    - `terms` table
        - `term_id` (PK)
        - `term` (string)
        - `type` (enum)
            - Verb
            - Adj.
            - etc.
        - Gender/Plurality (enum)
        - Gender variations (JSON)
        - CEFR level (enum)
        - Pejorative (bool)
        - Colloquial (bool)
        - Vulgar (bool)
        - Profanity (bool)
        - `ipa_pronunciation` (string)
        - `tags` (string, comma-separated)
            - TBD: need some way to index these
    - `term_definitions` table
        - `term_id` (foreign key from `terms`)
        - `definition_id` (foreign key from `definitions`)
    - `definitions` table
        - `definition_id` (PK)
        - `short_definition` (string)
        - `long_definition` (string)
    - `term_links` table
        - `term_a_id` (foreign key from `terms`)
        - `term_b_id` (foreign key from `terms`)
        - `link_type` (enum)
            - Synonym
            - Related term (e.g. verb <-> adj., etc.)
            - Antonym (not part of MVP)
            - etc.
    - `collocations` table (not part of MVP)
        - `term_id` (foreign key from `terms`)
        - TODO format
- GraphQL for querying SQLite database in Rust
- Script for quickly populating SQLite database from CSV

### Review Game (Frontend App)

- Allow filtering based on CEFR level, types of terms, profanity/vulgar inclusion or exclusion (persisted in account settings)
- All French words can be pronounced using text-to-speech
- Modes (chosen at random, each mode can be enabled or disabled, the settings for which are persisted in account settings):
    - Present English definition:
        1. **Translation (Text input):** Require typing in French term
        2. **Translation (Multiple choice):** Pick from 4 French terms with the same part of speech/type
        3. **Translation (Voice):** (Skippable, falls back to 1 of the other modes) voice-to-text pronunciation of French term
    - Present French term:
        1. **Pronunciation (Voice):** (Skippable, falls back to 1 of the other modes) voice-to-text pronunciation of French term
        2. **Translation (Multiple choice):** Pick from 4 English definitions with the same part of speech/type
        3. **Synonym (Text input):** Type in a French synonym
        4. **Synonym (Multiple Choice):** Pick from 4 possible French synonyms with the same part of speech/type
        5. **Use In A Sentence (Text input):** Type in sentence using the term, verifying grammar and/or translation somehow
        6. **Use In A Sentence (Voice):** (Skippable, falls back to 1 of the other modes) voice-to-text sentence construction using the term, verifying grammar and/or translation somehow
- Allow viewing any words' full info in side panel by clicking

### Database Edit (Frontend App)

- Add new term functionality
- Edit existing term functionality (edit fields, add term links)

### Account Features

- Google Account Login
- Persist Review Game Settings In Account
- Store Per-Word/Per-Phrase Stats In Account

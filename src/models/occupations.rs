use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum Occupation {
    Animator,
    Arranger,
    Artist,
    AudioEngineer,
    BackgroundArtist,
    Band,
    CGArtist,
    Composer,
    CompositeArtist,
    Designer,
    Director,
    Editor,
    Illustrator,
    Lyricist,
    Mangaka,
    Musician,
    Painter,
    Producer,
    ProductionManager,
    ScriptWriter,
    StoryBoardArtist,
    Translator,
    Vocalist,
    VoiceActor,
    Writer,
    #[default]
    None,
}

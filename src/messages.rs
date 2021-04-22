use serde::{Deserialize, Serialize};
use serde_diff::{SerdeDiff, Diff, Apply};
use oauth2::{AuthorizationCode, RefreshToken};
use std::time::Duration;
use url::Url;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Message {
    ClientRequest(ClientRequest),
    Auth(AuthMessage),
    State(State),
    Unexpected()
}

pub enum PatchError{
    DecodeError(rmp_serde::decode::Error),
    WrongVariant()
}

impl Message{
    pub fn patch_player_state(&self, state: &mut PlayerState) -> Result<(), PatchError>{
        if let Message::State(State::UpdateState(patch)) = self{
            let mut de = rmp_serde::Deserializer::new(patch.as_slice());
            return Apply::apply(&mut de, state).map_err(PatchError::DecodeError);
        }

        Err(PatchError::WrongVariant())
    }

    pub fn generate_patch(old: &PlayerState, new: &PlayerState) -> Result<Vec<u8>, rmp_serde::encode::Error>{
        rmp_serde::to_vec(&Diff::serializable(old, new))
    }

    pub fn generate(&self) -> Result<Vec<u8>, rmp_serde::encode::Error>{
        rmp_serde::to_vec(self)
    }

    pub fn parse(bin: &[u8]) -> Result<Self, rmp_serde::decode::Error>{
        rmp_serde::from_read(bin)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Unexpected{
    WsMessageTypeString(),
    ParseError()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum AuthMessage{
    AuthStatus(bool),
    AuthSuccess(User),
    AuthError(),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum State{
    FullState(Box<PlayerState>),
    UpdateState(Vec<u8>)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ClientRequest {
    Authenticate(Auth),
    AuthStatus(),
    FullPlayerState(),
    Control(PlayerControl),
    End()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Auth{
    Code(AuthorizationCode),
    Token(RefreshToken),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum PlayerControl{
    Resume(),
    Pause(),
    Skip(usize),
    BackSkip(usize),
    SetTime(Duration),
    PlayMode(PlayMode),
    Enqueue(Url),
    Leave(),
    Join(),
}


#[derive(Debug, Deserialize, Serialize, SerdeDiff, Clone, PartialEq)]
pub enum PlayMode{
    Normal,
    LoopAll,
    LoopOne,
}

#[derive(Debug, Deserialize, Serialize, SerdeDiff, Clone, PartialEq)]
pub struct PlayerState{
    pub bot: BotInfo,
    pub paused: bool,
    pub mode: PlayMode,
    pub current: Option<Track>,
    pub history: Vec<Track>,
    pub queue: Vec<Track>,
}

#[derive(Debug, Deserialize, Serialize, SerdeDiff, Clone, PartialEq)]
pub struct BotInfo{
    pub name: String,
    pub avatar: String,
}

#[derive(Debug, Deserialize, Serialize, SerdeDiff, Clone, PartialEq)]
pub struct Track{
    pub len: Duration,
    pub pos: Duration,
    pub title: String,
    pub uri: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct User{
    pub username: String,
    pub id: String,
    pub avatar: String,
}
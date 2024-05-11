mod archive;
mod authentication;
mod avatar;
mod battle;
mod events;
mod friend;
mod gacha;
mod lineup;
mod mail;
mod mission;
mod npc;
mod player;
mod scene;
mod tutorial;

use anyhow::Result;
use paste::paste;
use proto::*;
use tokio::io::AsyncWriteExt;

use super::PlayerSession;
use crate::net::NetPacket;

pub use archive::*;
pub use authentication::*;
pub use avatar::*;
pub use battle::*;
pub use events::*;
pub use friend::*;
pub use gacha::*;
pub use lineup::*;
pub use mail::*;
pub use mission::*;
pub use npc::*;
pub use player::*;
pub use scene::*;
pub use tutorial::*;

#[allow(unused_imports)]
use proto::{
    CmdActivityType::*, CmdAdventureType::*, CmdAetherDivideType::*, CmdAlleyType::*,
    CmdArchiveType::*, CmdAvatarType::*, CmdBattleCollegeType::*, CmdBattlePassType::*,
    CmdBattleType::*, CmdBoxingClubType::*, CmdChallengeType::*, CmdChatType::*,
    CmdChessRogueType::*, CmdClockParkType::*, CmdContentPackageType::*, CmdDailyActiveType::*,
    CmdDrinkMakerType::*, CmdEvolveBuildType::*, CmdExpeditionType::*,
    CmdFantasticStoryActivityType::*, CmdFeverTimeActivityType::*, CmdFightActivityType::*,
    CmdFightMathc3Type::*, CmdFightType::*, CmdFriendType::*, CmdGachaType::*, CmdHeartdialType::*,
    CmdHeliobusType::*, CmdItemType::*, CmdJukeboxType::*, CmdLineupType::*, CmdLobbyType::*,
    CmdMailType::*, CmdMapRotationType::*, CmdMatchThreeModuleType::*, CmdMatchType::*,
    CmdMessageType::*, CmdMiscModuleType::*, CmdMissionType::*, CmdMonopolyType::*,
    CmdMultiplayerType::*, CmdMultipleDropType::*, CmdMuseumType::*, CmdOfferingType::*,
    CmdPamMissionType::*, CmdPhoneType::*, CmdPlayerBoardType::*, CmdPlayerReturnType::*,
    CmdPlayerSync::*, CmdPlayerType::*, CmdPlotType::*, CmdPunkLordType::*, CmdQuestType::*,
    CmdRaidCollectionType::*, CmdRaidType::*, CmdRedDotType::*, CmdReplayType::*,
    CmdRndOptionType::*, CmdRogueCommonType::*, CmdRogueEndless::*, CmdRogueModifierType::*,
    CmdRogueTournType::*, CmdRogueType::*, CmdRollShopType::*, CmdSceneType::*,
    CmdServerPrefsType::*, CmdShopType::*, CmdSpaceZooType::*, CmdStarFightType::*,
    CmdStoryLineType::*, CmdStrongChallengeActivityType::*, CmdTalkRewardType::*,
    CmdTelevisionActivityType::*, CmdTextJoinType::*, CmdTrainVisitorType::*,
    CmdTravelBrochureType::*, CmdTreasureDungeonType::*, CmdTutorialType::*, CmdWaypointType::*,
    CmdWolfBroType::*
};

macro_rules! dummy {
    ($($cmd:ident),* $(,)*) => {
        paste! {
            impl PlayerSession {
                pub const fn should_send_dummy_rsp(cmd_id: u16) -> bool {
                    match cmd_id {
                        $(
                            x if x == [<Cmd $cmd CsReq>] as u16 => true,
                        )*
                        _ => false,
                    }
                }

                pub async fn send_dummy_response(&self, req_id: u16) -> Result<()> {
                    let cmd_type = match req_id {
                        $(
                            x if x == [<Cmd $cmd CsReq>] as u16 => [<Cmd $cmd ScRsp>] as u16,
                        )*
                        _ => return Err(anyhow::anyhow!("Invalid request id {req_id:?}")),
                    };

                    tracing::info!("Sent dummy response: {req_id}");

                    let payload: Vec<u8> = NetPacket {
                        cmd_type,
                        head: Vec::new(),
                        body: Vec::new(),
                    }
                    .into();

                    self.client_socket().await.write_all(&payload).await?;

                    Ok(())
                }
            }
        }
    };
}


dummy! {
    UpdateServerPrefsData,
    // SceneEntityMove,
    GetMarkItemList,
    GetAllServerPrefsData,
    GetLevelRewardTakenList,
    GetRogueScoreRewardInfo,
    GetRogueCommonDialogueData,
    GetRogueEndlessActivityData,
    GetMonsterResearchActivityData,
    GetMainMissionCustomValue,
    // GetGachaInfo,
    QueryProductInfo,
    GetQuestData,
    GetQuestRecord,
    // GetFriendListInfo,
    GetFriendApplyListInfo,
    GetCurAssist,
    GetRogueHandbookData,
    GetDailyActiveInfo,
    GetFightActivityData,
    GetMultipleDropInfo,
    GetPlayerReturnMultiDropInfo,
    GetShareData,
    GetTreasureDungeonActivityData,
    PlayerReturnInfoQuery,
    // GetBag,
    GetPlayerBoardData,
    // GetActivityScheduleConfig,
    GetMissionData,
    GetMissionEventData,
    GetChallenge,
    GetCurChallenge,
    GetRogueInfo,
    GetExpeditionData,
    // GetJukeboxData,
    SyncClientResVersion,
    DailyFirstMeetPam,
    GetMuseumInfo,
    GetLoginActivity,
    GetRaidInfo,
    GetTrialActivityData,
    GetBoxingClubInfo,
    GetNpcStatus,
    TextJoinQuery,
    GetSpringRecoverData,
    GetChatFriendHistory,
    GetSecretKeyInfo,
    GetVideoVersionKey,
    GetCurBattleInfo,
    // GetPhoneData, 
    PlayerLoginFinish,
    RogueTournQuery,
    GetBattleCollegeData,
    GetHeartDialInfo,
    HeliobusActivityData,
    GetEnteredScene,
    GetAetherDivideInfo,
    GetMapRotationData,
    GetRogueCollection,
    GetRogueExhibition,
    GetNpcMessageGroup,
    GetFriendLoginInfo,
    GetChessRogueNousStoryInfo,
    CommonRogueQuery,
    GetStarFightData,
    EvolveBuildQueryInfo,
    GetAlleyInfo,
    GetAetherDivideChallengeInfo,
    GetStrongChallengeActivityData,
    GetOfferingInfo,
    ClockParkGetInfo,
    GetGunPlayData,
    SpaceZooData,
    GetUnlockTeleport,
    // TravelBrochureGetData,
    RaidCollectionData,
    GetChatEmojiList,
    GetTelevisionActivityData,
    GetTrainVisitorRegister,
    GetLoginChatInfo,
    GetFeverTimeActivityData,
    GetAllSaveRaid,
}
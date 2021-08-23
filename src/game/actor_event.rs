use crate::util::{ProtocolEncoder, ProtocolDecoder};
use binary_utils::{BinaryStream, IBufferRead, IBufferWrite};
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Copy, Clone)]
pub enum ActorEventType {
     Jump = 1,
     Hurt,
     Death,
     StartAttack,
     StopAttack,
     TameFail,
     TameSuccess,
     ShakeWet,
     UseItem,
     EatGrass,
     HookBubble,
     HookPosition,
     HookRetrieve,
     HookTease,
     SquidInkCloud,
     CureVillager,
     Respawn,
     IronGolemOffer,
     IronGolemOfferStop,
     Mate,
     HappyVillager,
     AngryVillager,
     WitchSpell,
     Firework,
     FoundPartner,
     SilverfishSpawn,
     GuardianAttack,
     WitchDrink,
     WitchThrow,
     MinecartTntPrime,
     CreeperPrime,
     AirSupplyExpired,
     PlayerAddXpLevel,
     ElderGuardianCurse,
     AgentArmSwing,
     EnderDragonDeath,
     DustParticle,
     ArrowShake,
     EatItem = 57,
     FeedBabyAnimal = 60,
     DeathSmokeCloud,
     CompleteTrade,
     RemoveLeash,
     LlamaCaravanUpdated,
     ConsumeTotem,
     PlayerCheckTreasureHunterAchievement,
     EntitySpawn,
     DragonBreath,
     ItemEntityMerge,
     StartSwimming,
     BalloonPop,
     TreasureHunt,
     SummonAgent,
     CrossbowCharge
}

impl TryFrom<u8> for ActorEventType {
     type Error = ();

     fn try_from(v: u8) -> Result<Self, Self::Error> {
          match v {
               x if x == ActorEventType::Jump as u8 => Ok(ActorEventType::Jump),
               x if x == ActorEventType::Hurt as u8 => Ok(ActorEventType::Hurt),
               x if x == ActorEventType::Death as u8 => Ok(ActorEventType::Death),
               x if x == ActorEventType::StartAttack as u8 => Ok(ActorEventType::StartAttack),
               x if x == ActorEventType::StopAttack as u8 => Ok(ActorEventType::StopAttack),
               x if x == ActorEventType::TameFail as u8 => Ok(ActorEventType::TameFail),
               x if x == ActorEventType::TameSuccess as u8 => Ok(ActorEventType::TameSuccess),
               x if x == ActorEventType::ShakeWet as u8 => Ok(ActorEventType::ShakeWet),
               x if x == ActorEventType::UseItem as u8 => Ok(ActorEventType::UseItem),
               x if x == ActorEventType::EatGrass as u8 => Ok(ActorEventType::EatGrass),
               x if x == ActorEventType::HookBubble as u8 => Ok(ActorEventType::HookBubble),
               x if x == ActorEventType::HookPosition as u8 => Ok(ActorEventType::HookPosition),
               x if x == ActorEventType::HookRetrieve as u8 => Ok(ActorEventType::HookRetrieve),
               x if x == ActorEventType::HookTease as u8 => Ok(ActorEventType::HookTease),
               x if x == ActorEventType::SquidInkCloud as u8 => Ok(ActorEventType::SquidInkCloud),
               x if x == ActorEventType::CureVillager as u8 => Ok(ActorEventType::CureVillager),
               x if x == ActorEventType::Respawn as u8 => Ok(ActorEventType::Respawn),
               x if x == ActorEventType::IronGolemOffer as u8 => Ok(ActorEventType::IronGolemOffer),
               x if x == ActorEventType::IronGolemOfferStop as u8 => Ok(ActorEventType::IronGolemOfferStop),
               x if x == ActorEventType::Mate as u8 => Ok(ActorEventType::Mate),
               x if x == ActorEventType::HappyVillager as u8 => Ok(ActorEventType::HappyVillager),
               x if x == ActorEventType::AngryVillager as u8 => Ok(ActorEventType::AngryVillager),
               x if x == ActorEventType::WitchSpell as u8 => Ok(ActorEventType::WitchSpell),
               x if x == ActorEventType::Firework as u8 => Ok(ActorEventType::Firework),
               x if x == ActorEventType::FoundPartner as u8 => Ok(ActorEventType::FoundPartner),
               x if x == ActorEventType::SilverfishSpawn as u8 => Ok(ActorEventType::SilverfishSpawn),
               x if x == ActorEventType::GuardianAttack as u8 => Ok(ActorEventType::GuardianAttack),
               x if x == ActorEventType::WitchDrink as u8 => Ok(ActorEventType::WitchDrink),
               x if x == ActorEventType::WitchThrow as u8 => Ok(ActorEventType::WitchThrow),
               x if x == ActorEventType::MinecartTntPrime as u8 => Ok(ActorEventType::MinecartTntPrime),
               x if x == ActorEventType::CreeperPrime as u8 => Ok(ActorEventType::CreeperPrime),
               x if x == ActorEventType::AirSupplyExpired as u8 => Ok(ActorEventType::AirSupplyExpired),
               x if x == ActorEventType::PlayerAddXpLevel as u8 => Ok(ActorEventType::PlayerAddXpLevel),
               x if x == ActorEventType::ElderGuardianCurse as u8 => Ok(ActorEventType::ElderGuardianCurse),
               x if x == ActorEventType::AgentArmSwing as u8 => Ok(ActorEventType::AgentArmSwing),
               x if x == ActorEventType::EnderDragonDeath as u8 => Ok(ActorEventType::EnderDragonDeath),
               x if x == ActorEventType::DustParticle as u8 => Ok(ActorEventType::DustParticle),
               x if x == ActorEventType::ArrowShake as u8 => Ok(ActorEventType::ArrowShake),
               x if x == ActorEventType::EatItem as u8 => Ok(ActorEventType::EatItem),
               x if x == ActorEventType::FeedBabyAnimal as u8 => Ok(ActorEventType::FeedBabyAnimal),
               x if x == ActorEventType::DeathSmokeCloud as u8 => Ok(ActorEventType::DeathSmokeCloud),
               x if x == ActorEventType::CompleteTrade as u8 => Ok(ActorEventType::CompleteTrade),
               x if x == ActorEventType::RemoveLeash as u8 => Ok(ActorEventType::RemoveLeash),
               x if x == ActorEventType::LlamaCaravanUpdated as u8 => Ok(ActorEventType::LlamaCaravanUpdated),
               x if x == ActorEventType::ConsumeTotem as u8 => Ok(ActorEventType::ConsumeTotem),
               x if x == ActorEventType::PlayerCheckTreasureHunterAchievement as u8 => Ok(ActorEventType::PlayerCheckTreasureHunterAchievement),
               x if x == ActorEventType::EntitySpawn as u8 => Ok(ActorEventType::EntitySpawn),
               x if x == ActorEventType::DragonBreath as u8 => Ok(ActorEventType::DragonBreath),
               x if x == ActorEventType::ItemEntityMerge as u8 => Ok(ActorEventType::ItemEntityMerge),
               x if x == ActorEventType::StartSwimming as u8 => Ok(ActorEventType::StartSwimming),
               x if x == ActorEventType::BalloonPop as u8 => Ok(ActorEventType::BalloonPop),
               x if x == ActorEventType::TreasureHunt as u8 => Ok(ActorEventType::TreasureHunt),
               x if x == ActorEventType::SummonAgent as u8 => Ok(ActorEventType::SummonAgent),
               x if x == ActorEventType::CrossbowCharge as u8 => Ok(ActorEventType::CrossbowCharge),
               _ => Err(())
          }
     }
}

impl ProtocolEncoder for ActorEventType {
     fn write(&self) -> BinaryStream {
          let mut stream = BinaryStream::new();
          stream.write_byte(*self as u8);
          stream
     }
}

pub struct ActorEvent {
     entity_id: u64,
     event_type: ActorEventType,
     data: i32
}

impl ProtocolEncoder for ActorEvent {
     fn write(&self) -> BinaryStream {
          // TODO Add proper implmentation for this!
          let mut stream = BinaryStream::new();
          stream.write_uvar_long(self.entity_id);
          stream.write_byte(self.event_type as u8);
          stream.write_var_int(self.data);
          stream
     }
}

impl ProtocolDecoder for ActorEvent {
     fn read(stream: &mut BinaryStream) -> Self {
          Self {
               entity_id: stream.read_uvar_long(),
               event_type: stream.read_byte().try_into().unwrap(),
               data: stream.read_var_int()
          }
     }
}
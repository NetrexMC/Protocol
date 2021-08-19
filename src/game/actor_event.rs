use crate::util::{ProtocolEncoder, ProtocolDecoder};
use binary_utils::{BinaryStream, IBinaryStream, IBufferRead, IBufferWrite};

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

impl ProtocolEncoder for ActorEventType {
     fn write(&mut self) -> BinaryStream {
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
     fn write(&mut self) -> BinaryStream {
          // TODO Add proper implmentation for this!
          let mut stream = BinaryStream::new();
          stream
     }
}
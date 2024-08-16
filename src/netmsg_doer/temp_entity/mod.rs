use crate::types::{TeBeamPoints, TeBspDecal, TeTextMessage, TempEntity};

use super::*;

macro_rules! wrap_ent {
    ($te:ident, $parser:ident, $input:ident, $aux:ident) => {{
        let ($input, res) = $parser::parse($input, $aux)?;
        ($input, TempEntity::$te(res))
    }};
}

mod beam_points;
mod bsp_decal;
mod text_message;

impl Doer for SvcTempEntity {
    fn id(&self) -> u8 {
        23
    }

    fn parse<'a>(i: &'a [u8], aux: &'a RefCell<Aux>) -> Result<'a, Self> {
        let (i, entity_type) = le_u8(i)?;

        let (i, entity) = match entity_type {
            0 => wrap_ent!(TeBeamPoints, TeBeamPoints, i, aux),
            1 => map(take(20usize), |res: &[u8]| {
                TempEntity::TeBeamEntPoint(res.to_vec())
            })(i)?,
            2 => map(take(6usize), |res: &[u8]| {
                TempEntity::TeGunshot(res.to_vec())
            })(i)?,
            // The docs say 6 but its parser says 11.
            3 => map(take(11usize), |res: &[u8]| {
                TempEntity::TeExplosion(res.to_vec())
            })(i)?,
            4 => map(take(6usize), |res: &[u8]| {
                TempEntity::TeTarExplosion(res.to_vec())
            })(i)?,
            5 => map(take(10usize), |res: &[u8]| {
                TempEntity::TeSmoke(res.to_vec())
            })(i)?,
            6 => map(take(12usize), |res: &[u8]| {
                TempEntity::TeTracer(res.to_vec())
            })(i)?,
            7 => map(take(17usize), |res: &[u8]| {
                TempEntity::TeLightning(res.to_vec())
            })(i)?,
            8 => map(take(16usize), |res: &[u8]| {
                TempEntity::TeBeamEnts(res.to_vec())
            })(i)?,
            9 => map(take(6usize), |res: &[u8]| {
                TempEntity::TeSparks(res.to_vec())
            })(i)?,
            10 => map(take(6usize), |res: &[u8]| {
                TempEntity::TeLavaSplash(res.to_vec())
            })(i)?,
            11 => map(take(6usize), |res: &[u8]| {
                TempEntity::TeTeleport(res.to_vec())
            })(i)?,
            12 => map(take(8usize), |res: &[u8]| {
                TempEntity::TeExplosion2(res.to_vec())
            })(i)?,
            13 => wrap_ent!(TeBspDecal, TeBspDecal, i, aux),
            14 => map(take(9usize), |res: &[u8]| {
                TempEntity::TeImplosion(res.to_vec())
            })(i)?,
            15 => map(take(19usize), |res: &[u8]| {
                TempEntity::TeSpriteTrail(res.to_vec())
            })(i)?,
            17 => map(take(10usize), |res: &[u8]| {
                TempEntity::TeSprite(res.to_vec())
            })(i)?,
            18 => map(take(16usize), |res: &[u8]| {
                TempEntity::TeBeamSprite(res.to_vec())
            })(i)?,
            19 => map(take(24usize), |res: &[u8]| {
                TempEntity::TeBeamTorus(res.to_vec())
            })(i)?,
            20 => map(take(24usize), |res: &[u8]| {
                TempEntity::TeBeamDisk(res.to_vec())
            })(i)?,
            21 => map(take(24usize), |res: &[u8]| {
                TempEntity::TeBeamCylinder(res.to_vec())
            })(i)?,
            22 => map(take(10usize), |res: &[u8]| {
                TempEntity::TeBeamFollow(res.to_vec())
            })(i)?,
            23 => map(take(11usize), |res: &[u8]| {
                TempEntity::TeGlowSprite(res.to_vec())
            })(i)?,
            24 => map(take(16usize), |res: &[u8]| {
                TempEntity::TeBeamRing(res.to_vec())
            })(i)?,
            25 => map(take(19usize), |res: &[u8]| {
                TempEntity::TeStreakSplash(res.to_vec())
            })(i)?,
            27 => map(take(12usize), |res: &[u8]| {
                TempEntity::TeDLight(res.to_vec())
            })(i)?,
            28 => map(take(16usize), |res: &[u8]| {
                TempEntity::TeELight(res.to_vec())
            })(i)?,
            29 => wrap_ent!(TeTextMessage, TeTextMessage, i, aux),
            30 => map(take(17usize), |res: &[u8]| TempEntity::TeLine(res.to_vec()))(i)?,
            31 => map(take(17usize), |res: &[u8]| TempEntity::TeBox(res.to_vec()))(i)?,
            99 => map(take(2usize), |res: &[u8]| {
                TempEntity::TeKillBeam(res.to_vec())
            })(i)?,
            100 => map(take(10usize), |res: &[u8]| {
                TempEntity::TeLargeFunnel(res.to_vec())
            })(i)?,
            101 => map(take(14usize), |res: &[u8]| {
                TempEntity::TeBloodStream(res.to_vec())
            })(i)?,
            102 => map(take(12usize), |res: &[u8]| {
                TempEntity::TeShowLine(res.to_vec())
            })(i)?,
            103 => map(take(14usize), |res: &[u8]| {
                TempEntity::TeBlood(res.to_vec())
            })(i)?,
            104 => map(take(9usize), |res: &[u8]| TempEntity::TeDecal(res.to_vec()))(i)?,
            105 => map(take(5usize), |res: &[u8]| TempEntity::TeFizz(res.to_vec()))(i)?,
            106 => map(take(17usize), |res: &[u8]| {
                TempEntity::TeModel(res.to_vec())
            })(i)?,
            107 => map(take(13usize), |res: &[u8]| {
                TempEntity::TeExplodeModel(res.to_vec())
            })(i)?,
            // Docs say 13 but its parser says 24.
            108 => map(take(24usize), |res: &[u8]| {
                TempEntity::TeBreakModel(res.to_vec())
            })(i)?,
            109 => map(take(9usize), |res: &[u8]| {
                TempEntity::TeGunshotDecal(res.to_vec())
            })(i)?,
            110 => map(take(17usize), |res: &[u8]| {
                TempEntity::TeSpriteSpray(res.to_vec())
            })(i)?,
            111 => map(take(7usize), |res: &[u8]| {
                TempEntity::TeArmorRicochet(res.to_vec())
            })(i)?,
            112 => map(take(10usize), |res: &[u8]| {
                TempEntity::TePlayerDecal(res.to_vec())
            })(i)?,
            113 => map(take(10usize), |res: &[u8]| {
                TempEntity::TeBubbles(res.to_vec())
            })(i)?,
            114 => map(take(19usize), |res: &[u8]| {
                TempEntity::TeBubbleTrail(res.to_vec())
            })(i)?,
            115 => map(take(12usize), |res: &[u8]| {
                TempEntity::TeBloodSprite(res.to_vec())
            })(i)?,
            116 => map(take(7usize), |res: &[u8]| {
                TempEntity::TeWorldDecal(res.to_vec())
            })(i)?,
            117 => map(take(7usize), |res: &[u8]| {
                TempEntity::TeWorldDecalHigh(res.to_vec())
            })(i)?,
            118 => map(take(9usize), |res: &[u8]| {
                TempEntity::TeDecalHigh(res.to_vec())
            })(i)?,
            119 => map(take(16usize), |res: &[u8]| {
                TempEntity::TeProjectile(res.to_vec())
            })(i)?,
            120 => map(take(18usize), |res: &[u8]| {
                TempEntity::TeSpray(res.to_vec())
            })(i)?,
            121 => map(take(5usize), |res: &[u8]| {
                TempEntity::TePlayerSprites(res.to_vec())
            })(i)?,
            122 => map(take(10usize), |res: &[u8]| {
                TempEntity::TeParticleBurst(res.to_vec())
            })(i)?,
            123 => map(take(9usize), |res: &[u8]| {
                TempEntity::TeFireField(res.to_vec())
            })(i)?,
            124 => map(take(7usize), |res: &[u8]| {
                TempEntity::TePlayerAttachment(res.to_vec())
            })(i)?,
            125 => map(take(1usize), |res: &[u8]| {
                TempEntity::TeKillPlayerAttachment(res.to_vec())
            })(i)?,
            // Docs say 10 but its parser says 18.
            126 => map(take(18usize), |res: &[u8]| {
                TempEntity::TeMultigunShot(res.to_vec())
            })(i)?,
            127 => map(take(15usize), |res: &[u8]| {
                TempEntity::TeUserTracer(res.to_vec())
            })(i)?,
            _ => context("Bad temp entity number", fail)(i)?,
        };

        Ok((
            i,
            Self {
                entity_type,
                entity,
            },
        ))
    }

    fn write(&self, aux: &RefCell<Aux>) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());
        writer.append_u8(self.entity_type);

        let bytes = match &self.entity {
            TempEntity::TeBeamPoints(i) => &i.write(aux),
            TempEntity::TeBeamEntPoint(i) => i,
            TempEntity::TeGunshot(i) => i,
            TempEntity::TeExplosion(i) => i,
            TempEntity::TeTarExplosion(i) => i,
            TempEntity::TeSmoke(i) => i,
            TempEntity::TeTracer(i) => i,
            TempEntity::TeLightning(i) => i,
            TempEntity::TeBeamEnts(i) => i,
            TempEntity::TeSparks(i) => i,
            TempEntity::TeLavaSplash(i) => i,
            TempEntity::TeTeleport(i) => i,
            TempEntity::TeExplosion2(i) => i,
            TempEntity::TeBspDecal(i) => &i.write(aux),
            TempEntity::TeImplosion(i) => i,
            TempEntity::TeSpriteTrail(i) => i,
            TempEntity::TeSprite(i) => i,
            TempEntity::TeBeamSprite(i) => i,
            TempEntity::TeBeamTorus(i) => i,
            TempEntity::TeBeamDisk(i) => i,
            TempEntity::TeBeamCylinder(i) => i,
            TempEntity::TeBeamFollow(i) => i,
            TempEntity::TeGlowSprite(i) => i,
            TempEntity::TeBeamRing(i) => i,
            TempEntity::TeStreakSplash(i) => i,
            TempEntity::TeDLight(i) => i,
            TempEntity::TeELight(i) => i,
            TempEntity::TeTextMessage(i) => &i.write(aux),
            TempEntity::TeLine(i) => i,
            TempEntity::TeBox(i) => i,
            TempEntity::TeKillBeam(i) => i,
            TempEntity::TeLargeFunnel(i) => i,
            TempEntity::TeBloodStream(i) => i,
            TempEntity::TeShowLine(i) => i,
            TempEntity::TeBlood(i) => i,
            TempEntity::TeDecal(i) => i,
            TempEntity::TeFizz(i) => i,
            TempEntity::TeModel(i) => i,
            TempEntity::TeExplodeModel(i) => i,
            TempEntity::TeBreakModel(i) => i,
            TempEntity::TeGunshotDecal(i) => i,
            TempEntity::TeSpriteSpray(i) => i,
            TempEntity::TeArmorRicochet(i) => i,
            TempEntity::TePlayerDecal(i) => i,
            TempEntity::TeBubbles(i) => i,
            TempEntity::TeBubbleTrail(i) => i,
            TempEntity::TeBloodSprite(i) => i,
            TempEntity::TeWorldDecal(i) => i,
            TempEntity::TeWorldDecalHigh(i) => i,
            TempEntity::TeDecalHigh(i) => i,
            TempEntity::TeProjectile(i) => i,
            TempEntity::TeSpray(i) => i,
            TempEntity::TePlayerSprites(i) => i,
            TempEntity::TeParticleBurst(i) => i,
            TempEntity::TeFireField(i) => i,
            TempEntity::TePlayerAttachment(i) => i,
            TempEntity::TeKillPlayerAttachment(i) => i,
            TempEntity::TeMultigunShot(i) => i,
            TempEntity::TeUserTracer(i) => i,
        };

        writer.append_u8_slice(bytes);

        writer.data
    }
}

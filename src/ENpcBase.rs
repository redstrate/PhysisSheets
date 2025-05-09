#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ENpcBase {
exd: EXD,
exh: EXH,
}
impl ENpcBase {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ENpcBase").unwrap();let exd = game_data.read_excel_sheet("ENpcBase", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ENpcBaseRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ENpcBaseRow { columns: row.columns.clone() }
}
}
pub struct ENpcBaseRow {
columns: Vec<ColumnData>,
}
impl ENpcBaseRow {
pub fn ENpcData(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ModelMainHand(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ModelOffHand(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Scale(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ModelHead(&self) -> &ColumnData {
&self.columns[4]
}
pub fn ModelBody(&self) -> &ColumnData {
&self.columns[5]
}
pub fn ModelHands(&self) -> &ColumnData {
&self.columns[6]
}
pub fn ModelLegs(&self) -> &ColumnData {
&self.columns[7]
}
pub fn ModelFeet(&self) -> &ColumnData {
&self.columns[8]
}
pub fn ModelEars(&self) -> &ColumnData {
&self.columns[9]
}
pub fn ModelNeck(&self) -> &ColumnData {
&self.columns[10]
}
pub fn ModelWrists(&self) -> &ColumnData {
&self.columns[11]
}
pub fn ModelLeftRing(&self) -> &ColumnData {
&self.columns[12]
}
pub fn ModelRightRing(&self) -> &ColumnData {
&self.columns[13]
}
pub fn EventHandler(&self) -> &ColumnData {
&self.columns[14]
}
pub fn ModelChara(&self) -> &ColumnData {
&self.columns[15]
}
pub fn NpcEquip(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Behavior(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Unknown_70_1(&self) -> &ColumnData {
&self.columns[18]
}
pub fn Unknown_70_2(&self) -> &ColumnData {
&self.columns[19]
}
pub fn Balloon(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Race(&self) -> &ColumnData {
&self.columns[21]
}
pub fn Gender(&self) -> &ColumnData {
&self.columns[22]
}
pub fn BodyType(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Height(&self) -> &ColumnData {
&self.columns[24]
}
pub fn Tribe(&self) -> &ColumnData {
&self.columns[25]
}
pub fn Face(&self) -> &ColumnData {
&self.columns[26]
}
pub fn HairStyle(&self) -> &ColumnData {
&self.columns[27]
}
pub fn HairHighlight(&self) -> &ColumnData {
&self.columns[28]
}
pub fn SkinColor(&self) -> &ColumnData {
&self.columns[29]
}
pub fn EyeHeterochromia(&self) -> &ColumnData {
&self.columns[30]
}
pub fn HairColor(&self) -> &ColumnData {
&self.columns[31]
}
pub fn HairHighlightColor(&self) -> &ColumnData {
&self.columns[32]
}
pub fn FacialFeature(&self) -> &ColumnData {
&self.columns[33]
}
pub fn FacialFeatureColor(&self) -> &ColumnData {
&self.columns[34]
}
pub fn Eyebrows(&self) -> &ColumnData {
&self.columns[35]
}
pub fn EyeColor(&self) -> &ColumnData {
&self.columns[36]
}
pub fn EyeShape(&self) -> &ColumnData {
&self.columns[37]
}
pub fn Nose(&self) -> &ColumnData {
&self.columns[38]
}
pub fn Jaw(&self) -> &ColumnData {
&self.columns[39]
}
pub fn Mouth(&self) -> &ColumnData {
&self.columns[40]
}
pub fn LipColor(&self) -> &ColumnData {
&self.columns[41]
}
pub fn BustOrTone1(&self) -> &ColumnData {
&self.columns[42]
}
pub fn ExtraFeature1(&self) -> &ColumnData {
&self.columns[43]
}
pub fn ExtraFeature2OrBust(&self) -> &ColumnData {
&self.columns[44]
}
pub fn FacePaint(&self) -> &ColumnData {
&self.columns[45]
}
pub fn FacePaintColor(&self) -> &ColumnData {
&self.columns[46]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[47]
}
pub fn DyeMainHand(&self) -> &ColumnData {
&self.columns[48]
}
pub fn Dye2MainHand(&self) -> &ColumnData {
&self.columns[49]
}
pub fn DyeOffHand(&self) -> &ColumnData {
&self.columns[50]
}
pub fn Dye2OffHand(&self) -> &ColumnData {
&self.columns[51]
}
pub fn DyeHead(&self) -> &ColumnData {
&self.columns[52]
}
pub fn DyeBody(&self) -> &ColumnData {
&self.columns[53]
}
pub fn DyeHands(&self) -> &ColumnData {
&self.columns[54]
}
pub fn DyeLegs(&self) -> &ColumnData {
&self.columns[55]
}
pub fn DyeFeet(&self) -> &ColumnData {
&self.columns[56]
}
pub fn DyeEars(&self) -> &ColumnData {
&self.columns[57]
}
pub fn DyeNeck(&self) -> &ColumnData {
&self.columns[58]
}
pub fn DyeWrists(&self) -> &ColumnData {
&self.columns[59]
}
pub fn DyeLeftRing(&self) -> &ColumnData {
&self.columns[60]
}
pub fn DyeRightRing(&self) -> &ColumnData {
&self.columns[61]
}
pub fn Dye2Head(&self) -> &ColumnData {
&self.columns[62]
}
pub fn Dye2Body(&self) -> &ColumnData {
&self.columns[63]
}
pub fn Dye2Hands(&self) -> &ColumnData {
&self.columns[64]
}
pub fn Dye2Legs(&self) -> &ColumnData {
&self.columns[65]
}
pub fn Dye2Feet(&self) -> &ColumnData {
&self.columns[66]
}
pub fn Dye2Ears(&self) -> &ColumnData {
&self.columns[67]
}
pub fn Dye2Neck(&self) -> &ColumnData {
&self.columns[68]
}
pub fn Dye2Wrists(&self) -> &ColumnData {
&self.columns[69]
}
pub fn Dye2LeftRing(&self) -> &ColumnData {
&self.columns[70]
}
pub fn Dye2RightRing(&self) -> &ColumnData {
&self.columns[71]
}
pub fn Invisibility(&self) -> &ColumnData {
&self.columns[72]
}
pub fn DefaultBalloon(&self) -> &ColumnData {
&self.columns[73]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[74]
}
pub fn Important(&self) -> &ColumnData {
&self.columns[75]
}
pub fn Visor(&self) -> &ColumnData {
&self.columns[76]
}
pub fn NotRewriteHeight(&self) -> &ColumnData {
&self.columns[77]
}
}

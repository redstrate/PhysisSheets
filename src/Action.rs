#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct Action {
exd: EXD,
exh: EXH,
}
impl Action {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Action").unwrap();let exd = game_data.read_excel_sheet("Action", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ActionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
ActionRow { columns }
}
}
pub struct ActionRow {
columns: Vec<ColumnData>,
}
impl ActionRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn UnlockLink(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[2]
}
pub fn VFX(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ActionTimelineHit(&self) -> &ColumnData {
&self.columns[4]
}
pub fn PrimaryCostValue(&self) -> &ColumnData {
&self.columns[5]
}
pub fn SecondaryCostValue(&self) -> &ColumnData {
&self.columns[6]
}
pub fn ActionCombo(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Cast100ms(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Recast100ms(&self) -> &ColumnData {
&self.columns[9]
}
pub fn ActionProcStatus(&self) -> &ColumnData {
&self.columns[10]
}
pub fn StatusGainSelf(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Omen(&self) -> &ColumnData {
&self.columns[12]
}
pub fn OmenAlt(&self) -> &ColumnData {
&self.columns[13]
}
pub fn AnimationEnd(&self) -> &ColumnData {
&self.columns[14]
}
pub fn ActionCategory(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[16]
}
pub fn AnimationStart(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[18]
}
pub fn BehaviourType(&self) -> &ColumnData {
&self.columns[19]
}
pub fn ClassJobLevel(&self) -> &ColumnData {
&self.columns[20]
}
pub fn CastType(&self) -> &ColumnData {
&self.columns[21]
}
pub fn EffectRange(&self) -> &ColumnData {
&self.columns[22]
}
pub fn XAxisModifier(&self) -> &ColumnData {
&self.columns[23]
}
pub fn PrimaryCostType(&self) -> &ColumnData {
&self.columns[24]
}
pub fn SecondaryCostType(&self) -> &ColumnData {
&self.columns[25]
}
pub fn ExtraCastTime100ms(&self) -> &ColumnData {
&self.columns[26]
}
pub fn CooldownGroup(&self) -> &ColumnData {
&self.columns[27]
}
pub fn AdditionalCooldownGroup(&self) -> &ColumnData {
&self.columns[28]
}
pub fn MaxCharges(&self) -> &ColumnData {
&self.columns[29]
}
pub fn Aspect(&self) -> &ColumnData {
&self.columns[30]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[31]
}
pub fn ClassJobCategory(&self) -> &ColumnData {
&self.columns[32]
}
pub fn AutoAttackBehaviour(&self) -> &ColumnData {
&self.columns[33]
}
pub fn EquivalenceGroup(&self) -> &ColumnData {
&self.columns[34]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[35]
}
pub fn ClassJob(&self) -> &ColumnData {
&self.columns[36]
}
pub fn Range(&self) -> &ColumnData {
&self.columns[37]
}
pub fn DeadTargetBehaviour(&self) -> &ColumnData {
&self.columns[38]
}
pub fn AttackType(&self) -> &ColumnData {
&self.columns[39]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[40]
}
pub fn IsRoleAction(&self) -> &ColumnData {
&self.columns[41]
}
pub fn CanTargetSelf(&self) -> &ColumnData {
&self.columns[42]
}
pub fn CanTargetParty(&self) -> &ColumnData {
&self.columns[43]
}
pub fn CanTargetAlliance(&self) -> &ColumnData {
&self.columns[44]
}
pub fn CanTargetHostile(&self) -> &ColumnData {
&self.columns[45]
}
pub fn CanTargetAlly(&self) -> &ColumnData {
&self.columns[46]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[47]
}
pub fn TargetArea(&self) -> &ColumnData {
&self.columns[48]
}
pub fn CanTargetOwnPet(&self) -> &ColumnData {
&self.columns[49]
}
pub fn CanTargetPartyPet(&self) -> &ColumnData {
&self.columns[50]
}
pub fn RequiresLineOfSight(&self) -> &ColumnData {
&self.columns[51]
}
pub fn NeedToFaceTarget(&self) -> &ColumnData {
&self.columns[52]
}
pub fn Unknown14(&self) -> &ColumnData {
&self.columns[53]
}
pub fn PreservesCombo(&self) -> &ColumnData {
&self.columns[54]
}
pub fn Unknown15(&self) -> &ColumnData {
&self.columns[55]
}
pub fn AffectsPosition(&self) -> &ColumnData {
&self.columns[56]
}
pub fn IsPvP(&self) -> &ColumnData {
&self.columns[57]
}
pub fn Unknown16(&self) -> &ColumnData {
&self.columns[58]
}
pub fn LogCastMessage(&self) -> &ColumnData {
&self.columns[59]
}
pub fn Unknown18(&self) -> &ColumnData {
&self.columns[60]
}
pub fn LogMissMessage(&self) -> &ColumnData {
&self.columns[61]
}
pub fn LogActionMessage(&self) -> &ColumnData {
&self.columns[62]
}
pub fn Unknown21(&self) -> &ColumnData {
&self.columns[63]
}
pub fn Unknown22(&self) -> &ColumnData {
&self.columns[64]
}
pub fn Unknown23(&self) -> &ColumnData {
&self.columns[65]
}
pub fn CanUseWhileMounted(&self) -> &ColumnData {
&self.columns[66]
}
pub fn Unknown25(&self) -> &ColumnData {
&self.columns[67]
}
pub fn IsPlayerAction(&self) -> &ColumnData {
&self.columns[68]
}
pub fn Unknown27(&self) -> &ColumnData {
&self.columns[69]
}
}

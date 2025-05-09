#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct ItemSheet {
exd: EXD,
exh: EXH,
}
impl ItemSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("Item")?;let exd = game_data.read_excel_sheet("Item", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<ItemRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(ItemRow { columns })
}
}
pub struct ItemRow {
columns: Vec<ColumnData>,
}
impl ItemRow {
pub fn Singular(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Plural(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Name(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Adjective(&self) -> &ColumnData {
&self.columns[4]
}
pub fn PossessivePronoun(&self) -> &ColumnData {
&self.columns[5]
}
pub fn StartsWithVowel(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Pronoun(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Article(&self) -> &ColumnData {
&self.columns[9]
}
pub fn ModelMain(&self) -> &ColumnData {
&self.columns[10]
}
pub fn ModelSub(&self) -> &ColumnData {
&self.columns[11]
}
pub fn DamagePhys(&self) -> &ColumnData {
&self.columns[12]
}
pub fn DamageMag(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Delayms(&self) -> &ColumnData {
&self.columns[14]
}
pub fn BlockRate(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Block(&self) -> &ColumnData {
&self.columns[16]
}
pub fn DefensePhys(&self) -> &ColumnData {
&self.columns[17]
}
pub fn DefenseMag(&self) -> &ColumnData {
&self.columns[18]
}
pub fn BaseParamValue(&self) -> &ColumnData {
&self.columns[19]
}
pub fn BaseParamValueSpecial(&self) -> &ColumnData {
&self.columns[20]
}
pub fn LevelEquip(&self) -> &ColumnData {
&self.columns[21]
}
pub fn RequiredPvpRank(&self) -> &ColumnData {
&self.columns[22]
}
pub fn EquipRestriction(&self) -> &ColumnData {
&self.columns[23]
}
pub fn ClassJobCategory(&self) -> &ColumnData {
&self.columns[24]
}
pub fn GrandCompany(&self) -> &ColumnData {
&self.columns[25]
}
pub fn ItemSeries(&self) -> &ColumnData {
&self.columns[26]
}
pub fn BaseParamModifier(&self) -> &ColumnData {
&self.columns[27]
}
pub fn ClassJobUse(&self) -> &ColumnData {
&self.columns[28]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[29]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[30]
}
pub fn BaseParam(&self) -> &ColumnData {
&self.columns[31]
}
pub fn ItemSpecialBonus(&self) -> &ColumnData {
&self.columns[32]
}
pub fn ItemSpecialBonusParam(&self) -> &ColumnData {
&self.columns[33]
}
pub fn BaseParamSpecial(&self) -> &ColumnData {
&self.columns[34]
}
pub fn MaterializeType(&self) -> &ColumnData {
&self.columns[35]
}
pub fn MateriaSlotCount(&self) -> &ColumnData {
&self.columns[36]
}
pub fn SubStatCategory(&self) -> &ColumnData {
&self.columns[37]
}
pub fn IsAdvancedMeldingPermitted(&self) -> &ColumnData {
&self.columns[38]
}
pub fn IsPvP(&self) -> &ColumnData {
&self.columns[39]
}
pub fn IsGlamorous(&self) -> &ColumnData {
&self.columns[40]
}
pub fn AdditionalData(&self) -> &ColumnData {
&self.columns[41]
}
pub fn StackSize(&self) -> &ColumnData {
&self.columns[42]
}
pub fn PriceMid(&self) -> &ColumnData {
&self.columns[43]
}
pub fn PriceLow(&self) -> &ColumnData {
&self.columns[44]
}
pub fn ItemRepair(&self) -> &ColumnData {
&self.columns[45]
}
pub fn ItemGlamour(&self) -> &ColumnData {
&self.columns[46]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[47]
}
pub fn LevelItem(&self) -> &ColumnData {
&self.columns[48]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[49]
}
pub fn ItemAction(&self) -> &ColumnData {
&self.columns[50]
}
pub fn Cooldowns(&self) -> &ColumnData {
&self.columns[51]
}
pub fn Desynth(&self) -> &ColumnData {
&self.columns[52]
}
pub fn AetherialReduce(&self) -> &ColumnData {
&self.columns[53]
}
pub fn Rarity(&self) -> &ColumnData {
&self.columns[54]
}
pub fn FilterGroup(&self) -> &ColumnData {
&self.columns[55]
}
pub fn ItemUICategory(&self) -> &ColumnData {
&self.columns[56]
}
pub fn ItemSearchCategory(&self) -> &ColumnData {
&self.columns[57]
}
pub fn EquipSlotCategory(&self) -> &ColumnData {
&self.columns[58]
}
pub fn ItemSortCategory(&self) -> &ColumnData {
&self.columns[59]
}
pub fn DyeCount(&self) -> &ColumnData {
&self.columns[60]
}
pub fn CastTimeSeconds(&self) -> &ColumnData {
&self.columns[61]
}
pub fn ClassJobRepair(&self) -> &ColumnData {
&self.columns[62]
}
pub fn IsUnique(&self) -> &ColumnData {
&self.columns[63]
}
pub fn IsUntradable(&self) -> &ColumnData {
&self.columns[64]
}
pub fn IsIndisposable(&self) -> &ColumnData {
&self.columns[65]
}
pub fn Lot(&self) -> &ColumnData {
&self.columns[66]
}
pub fn CanBeHq(&self) -> &ColumnData {
&self.columns[67]
}
pub fn IsCrestWorthy(&self) -> &ColumnData {
&self.columns[68]
}
pub fn IsCollectable(&self) -> &ColumnData {
&self.columns[69]
}
pub fn AlwaysCollectable(&self) -> &ColumnData {
&self.columns[70]
}
}

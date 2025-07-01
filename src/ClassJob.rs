#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct ClassJobSheet {
exd: EXD,
exh: EXH,
}
impl ClassJobSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("ClassJob")?;let exd = game_data.read_excel_sheet("ClassJob", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<ClassJobRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(ClassJobRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<ClassJobRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<ClassJobRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct ClassJobRow {
columns: Vec<ColumnData>,
}
impl ClassJobRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Abbreviation(&self) -> &ColumnData {
&self.columns[1]
}
pub fn NameFemale(&self) -> &ColumnData {
&self.columns[2]
}
pub fn CanQueueForDuty(&self) -> &ColumnData {
&self.columns[3]
}
pub fn NameEnglish(&self) -> &ColumnData {
&self.columns[4]
}
pub fn ItemSoulCrystal(&self) -> &ColumnData {
&self.columns[5]
}
pub fn UnlockQuest(&self) -> &ColumnData {
&self.columns[6]
}
pub fn RelicQuest(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Prerequisite(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown_70_1(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown_70_2(&self) -> &ColumnData {
&self.columns[10]
}
pub fn ItemStartingWeapon(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[12]
}
pub fn ModifierHitPoints(&self) -> &ColumnData {
&self.columns[13]
}
pub fn ModifierManaPoints(&self) -> &ColumnData {
&self.columns[14]
}
pub fn ModifierStrength(&self) -> &ColumnData {
&self.columns[15]
}
pub fn ModifierVitality(&self) -> &ColumnData {
&self.columns[16]
}
pub fn ModifierDexterity(&self) -> &ColumnData {
&self.columns[17]
}
pub fn ModifierIntelligence(&self) -> &ColumnData {
&self.columns[18]
}
pub fn ModifierMind(&self) -> &ColumnData {
&self.columns[19]
}
pub fn ModifierPiety(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[21]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[22]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[24]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[25]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[26]
}
pub fn LimitBreak1(&self) -> &ColumnData {
&self.columns[27]
}
pub fn LimitBreak2(&self) -> &ColumnData {
&self.columns[28]
}
pub fn LimitBreak3(&self) -> &ColumnData {
&self.columns[29]
}
pub fn ClassJobCategory(&self) -> &ColumnData {
&self.columns[30]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[31]
}
pub fn JobIndex(&self) -> &ColumnData {
&self.columns[32]
}
pub fn PvPBaseParamValue(&self) -> &ColumnData {
&self.columns[33]
}
pub fn PvPActionSortRow(&self) -> &ColumnData {
&self.columns[34]
}
pub fn PvPInitialSelectActionTrait(&self) -> &ColumnData {
&self.columns[35]
}
pub fn ClassJobParent(&self) -> &ColumnData {
&self.columns[36]
}
pub fn Role(&self) -> &ColumnData {
&self.columns[37]
}
pub fn StartingTown(&self) -> &ColumnData {
&self.columns[38]
}
pub fn PrimaryStat(&self) -> &ColumnData {
&self.columns[39]
}
pub fn UIPriority(&self) -> &ColumnData {
&self.columns[40]
}
pub fn StartingLevel(&self) -> &ColumnData {
&self.columns[41]
}
pub fn PartyBonus(&self) -> &ColumnData {
&self.columns[42]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[43]
}
pub fn ExpArrayIndex(&self) -> &ColumnData {
&self.columns[44]
}
pub fn BattleClassIndex(&self) -> &ColumnData {
&self.columns[45]
}
pub fn DohDolJobIndex(&self) -> &ColumnData {
&self.columns[46]
}
pub fn MonsterNote(&self) -> &ColumnData {
&self.columns[47]
}
pub fn IsLimitedJob(&self) -> &ColumnData {
&self.columns[48]
}
}

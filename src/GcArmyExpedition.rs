#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct GcArmyExpeditionSheet {
exd: EXD,
exh: EXH,
}
impl GcArmyExpeditionSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("GcArmyExpedition")?;let exd = game_data.read_excel_sheet("GcArmyExpedition", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<GcArmyExpeditionRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(GcArmyExpeditionRow { columns })
}
}
pub struct GcArmyExpeditionRow {
columns: Vec<ColumnData>,
}
impl GcArmyExpeditionRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ExpeditionParams(&self) -> [&ColumnData; 6] {
[&self.columns[2],&self.columns[3],&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],]
}
pub fn RewardExperience(&self) -> &ColumnData {
&self.columns[8]
}
pub fn RequiredSeals(&self) -> &ColumnData {
&self.columns[9]
}
pub fn RequiredFlag(&self) -> &ColumnData {
&self.columns[10]
}
pub fn UnlockFlag(&self) -> &ColumnData {
&self.columns[11]
}
pub fn RequiredLevel(&self) -> &ColumnData {
&self.columns[12]
}
pub fn PercentBase(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[14]
}
pub fn GcArmyExpeditionType(&self) -> &ColumnData {
&self.columns[15]
}
}

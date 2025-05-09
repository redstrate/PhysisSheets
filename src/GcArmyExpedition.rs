#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GcArmyExpedition {
exd: EXD,
exh: EXH,
}
impl GcArmyExpedition {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GcArmyExpedition").unwrap();let exd = game_data.read_excel_sheet("GcArmyExpedition", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GcArmyExpeditionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GcArmyExpeditionRow { columns: row.columns.clone() }
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
pub fn ExpeditionParams(&self) -> &ColumnData {
&self.columns[2]
}
pub fn RewardExperience(&self) -> &ColumnData {
&self.columns[3]
}
pub fn RequiredSeals(&self) -> &ColumnData {
&self.columns[4]
}
pub fn RequiredFlag(&self) -> &ColumnData {
&self.columns[5]
}
pub fn UnlockFlag(&self) -> &ColumnData {
&self.columns[6]
}
pub fn RequiredLevel(&self) -> &ColumnData {
&self.columns[7]
}
pub fn PercentBase(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[9]
}
pub fn GcArmyExpeditionType(&self) -> &ColumnData {
&self.columns[10]
}
}

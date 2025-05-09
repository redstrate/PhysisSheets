#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GrandCompanyRank {
exd: EXD,
exh: EXH,
}
impl GrandCompanyRank {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GrandCompanyRank").unwrap();let exd = game_data.read_excel_sheet("GrandCompanyRank", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GrandCompanyRankRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GrandCompanyRankRow { columns: row.columns.clone() }
}
}
pub struct GrandCompanyRankRow {
columns: Vec<ColumnData>,
}
impl GrandCompanyRankRow {
pub fn MaxSeals(&self) -> &ColumnData {
&self.columns[0]
}
pub fn RequiredSeals(&self) -> &ColumnData {
&self.columns[1]
}
pub fn IconMaelstrom(&self) -> &ColumnData {
&self.columns[2]
}
pub fn IconSerpents(&self) -> &ColumnData {
&self.columns[3]
}
pub fn IconFlames(&self) -> &ColumnData {
&self.columns[4]
}
pub fn QuestMaelstrom(&self) -> &ColumnData {
&self.columns[5]
}
pub fn QuestSerpents(&self) -> &ColumnData {
&self.columns[6]
}
pub fn QuestFlames(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Tier(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[10]
}
}

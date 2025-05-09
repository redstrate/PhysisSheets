#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct NotoriousMonsterTerritory {
exd: EXD,
exh: EXH,
}
impl NotoriousMonsterTerritory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("NotoriousMonsterTerritory").unwrap();let exd = game_data.read_excel_sheet("NotoriousMonsterTerritory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> NotoriousMonsterTerritoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
NotoriousMonsterTerritoryRow { columns: row.columns.clone() }
}
}
pub struct NotoriousMonsterTerritoryRow {
columns: Vec<ColumnData>,
}
impl NotoriousMonsterTerritoryRow {
pub fn NotoriousMonsters(&self) -> &ColumnData {
&self.columns[0]
}
}

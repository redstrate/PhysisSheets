#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FCCrestSymbol {
exd: EXD,
exh: EXH,
}
impl FCCrestSymbol {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FCCrestSymbol").unwrap();let exd = game_data.read_excel_sheet("FCCrestSymbol", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FCCrestSymbolRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FCCrestSymbolRow { columns: row.columns.clone() }
}
}
pub struct FCCrestSymbolRow {
columns: Vec<ColumnData>,
}
impl FCCrestSymbolRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ColorNum(&self) -> &ColumnData {
&self.columns[1]
}
pub fn FCRight(&self) -> &ColumnData {
&self.columns[2]
}
}

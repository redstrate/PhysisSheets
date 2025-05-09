#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MapSymbol {
exd: EXD,
exh: EXH,
}
impl MapSymbol {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MapSymbol").unwrap();let exd = game_data.read_excel_sheet("MapSymbol", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MapSymbolRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MapSymbolRow { columns: row.columns.clone() }
}
}
pub struct MapSymbolRow {
columns: Vec<ColumnData>,
}
impl MapSymbolRow {
pub fn Icon(&self) -> &ColumnData {
&self.columns[0]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[1]
}
pub fn DisplayNavi(&self) -> &ColumnData {
&self.columns[2]
}
}

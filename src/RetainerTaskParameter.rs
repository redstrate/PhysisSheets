#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct RetainerTaskParameter {
exd: EXD,
exh: EXH,
}
impl RetainerTaskParameter {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("RetainerTaskParameter").unwrap();let exd = game_data.read_excel_sheet("RetainerTaskParameter", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RetainerTaskParameterRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
RetainerTaskParameterRow { columns: row.columns.clone() }
}
}
pub struct RetainerTaskParameterRow {
columns: Vec<ColumnData>,
}
impl RetainerTaskParameterRow {
pub fn ItemLevelDoW(&self) -> &ColumnData {
&self.columns[0]
}
pub fn PerceptionDoL(&self) -> &ColumnData {
&self.columns[1]
}
pub fn PerceptionFSH(&self) -> &ColumnData {
&self.columns[2]
}
}

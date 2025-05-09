#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct WarpLogic {
exd: EXD,
exh: EXH,
}
impl WarpLogic {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WarpLogic").unwrap();let exd = game_data.read_excel_sheet("WarpLogic", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WarpLogicRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
WarpLogicRow { columns: row.columns.clone() }
}
}
pub struct WarpLogicRow {
columns: Vec<ColumnData>,
}
impl WarpLogicRow {
pub fn WarpParams(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Question(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ResponseYes(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ResponseNo(&self) -> &ColumnData {
&self.columns[3]
}
pub fn WarpName(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[5]
}
pub fn CanSkipCutscene(&self) -> &ColumnData {
&self.columns[6]
}
}

#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MacroIconRedirectOld {
exd: EXD,
exh: EXH,
}
impl MacroIconRedirectOld {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MacroIconRedirectOld").unwrap();let exd = game_data.read_excel_sheet("MacroIconRedirectOld", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MacroIconRedirectOldRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MacroIconRedirectOldRow { columns: row.columns.clone() }
}
}
pub struct MacroIconRedirectOldRow {
columns: Vec<ColumnData>,
}
impl MacroIconRedirectOldRow {
pub fn IconOld(&self) -> &ColumnData {
&self.columns[0]
}
pub fn IconNew(&self) -> &ColumnData {
&self.columns[1]
}
}

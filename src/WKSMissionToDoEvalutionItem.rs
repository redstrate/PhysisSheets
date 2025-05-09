#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct WKSMissionToDoEvalutionItem {
exd: EXD,
exh: EXH,
}
impl WKSMissionToDoEvalutionItem {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WKSMissionToDoEvalutionItem").unwrap();let exd = game_data.read_excel_sheet("WKSMissionToDoEvalutionItem", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WKSMissionToDoEvalutionItemRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
WKSMissionToDoEvalutionItemRow { columns: row.columns.clone() }
}
}
pub struct WKSMissionToDoEvalutionItemRow {
columns: Vec<ColumnData>,
}
impl WKSMissionToDoEvalutionItemRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
}

#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MirageStoreSetItemLookup {
exd: EXD,
exh: EXH,
}
impl MirageStoreSetItemLookup {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MirageStoreSetItemLookup").unwrap();let exd = game_data.read_excel_sheet("MirageStoreSetItemLookup", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MirageStoreSetItemLookupRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MirageStoreSetItemLookupRow { columns: row.columns.clone() }
}
}
pub struct MirageStoreSetItemLookupRow {
columns: Vec<ColumnData>,
}
impl MirageStoreSetItemLookupRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
}

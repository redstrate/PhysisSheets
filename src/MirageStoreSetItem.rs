#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MirageStoreSetItem {
exd: EXD,
exh: EXH,
}
impl MirageStoreSetItem {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MirageStoreSetItem").unwrap();let exd = game_data.read_excel_sheet("MirageStoreSetItem", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MirageStoreSetItemRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MirageStoreSetItemRow { columns: row.columns.clone() }
}
}
pub struct MirageStoreSetItemRow {
columns: Vec<ColumnData>,
}
impl MirageStoreSetItemRow {
pub fn MainHand(&self) -> &ColumnData {
&self.columns[0]
}
pub fn OffHand(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Head(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Body(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Hands(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Legs(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Feet(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Earrings(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Necklace(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Bracelets(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Ring(&self) -> &ColumnData {
&self.columns[10]
}
}

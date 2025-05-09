#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ItemSearchCategory {
exd: EXD,
exh: EXH,
}
impl ItemSearchCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ItemSearchCategory").unwrap();let exd = game_data.read_excel_sheet("ItemSearchCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ItemSearchCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ItemSearchCategoryRow { columns: row.columns.clone() }
}
}
pub struct ItemSearchCategoryRow {
columns: Vec<ColumnData>,
}
impl ItemSearchCategoryRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Category(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ClassJob(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[5]
}
}

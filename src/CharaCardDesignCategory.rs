#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CharaCardDesignCategory {
exd: EXD,
exh: EXH,
}
impl CharaCardDesignCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CharaCardDesignCategory").unwrap();let exd = game_data.read_excel_sheet("CharaCardDesignCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CharaCardDesignCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CharaCardDesignCategoryRow { columns: row.columns.clone() }
}
}
pub struct CharaCardDesignCategoryRow {
columns: Vec<ColumnData>,
}
impl CharaCardDesignCategoryRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[1]
}
}

#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BannerDesignPreset {
exd: EXD,
exh: EXH,
}
impl BannerDesignPreset {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BannerDesignPreset").unwrap();let exd = game_data.read_excel_sheet("BannerDesignPreset", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BannerDesignPresetRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BannerDesignPresetRow { columns: row.columns.clone() }
}
}
pub struct BannerDesignPresetRow {
columns: Vec<ColumnData>,
}
impl BannerDesignPresetRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Background(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Frame(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Decoration(&self) -> &ColumnData {
&self.columns[3]
}
pub fn SortKey(&self) -> &ColumnData {
&self.columns[4]
}
}

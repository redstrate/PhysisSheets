#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BannerFrame {
exd: EXD,
exh: EXH,
}
impl BannerFrame {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BannerFrame").unwrap();let exd = game_data.read_excel_sheet("BannerFrame", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BannerFrameRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BannerFrameRow { columns: row.columns.clone() }
}
}
pub struct BannerFrameRow {
columns: Vec<ColumnData>,
}
impl BannerFrameRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Image(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[2]
}
pub fn UnlockCondition(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown_70_1(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown_70_2(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[6]
}
pub fn SortKey(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown_70_3(&self) -> &ColumnData {
&self.columns[8]
}
}

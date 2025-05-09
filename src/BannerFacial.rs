#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BannerFacial {
exd: EXD,
exh: EXH,
}
impl BannerFacial {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BannerFacial").unwrap();let exd = game_data.read_excel_sheet("BannerFacial", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BannerFacialRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BannerFacialRow { columns: row.columns.clone() }
}
}
pub struct BannerFacialRow {
columns: Vec<ColumnData>,
}
impl BannerFacialRow {
pub fn Emote(&self) -> &ColumnData {
&self.columns[0]
}
pub fn UnlockCondition(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown_70_1(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown_70_2(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[4]
}
pub fn SortKey(&self) -> &ColumnData {
&self.columns[5]
}
}

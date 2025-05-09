#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CharaCardDesignPreset {
exd: EXD,
exh: EXH,
}
impl CharaCardDesignPreset {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CharaCardDesignPreset").unwrap();let exd = game_data.read_excel_sheet("CharaCardDesignPreset", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CharaCardDesignPresetRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CharaCardDesignPresetRow { columns: row.columns.clone() }
}
}
pub struct CharaCardDesignPresetRow {
columns: Vec<ColumnData>,
}
impl CharaCardDesignPresetRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn BasePlate(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Backing(&self) -> &ColumnData {
&self.columns[2]
}
pub fn PatternOverlay(&self) -> &ColumnData {
&self.columns[3]
}
pub fn PortraitFrame(&self) -> &ColumnData {
&self.columns[4]
}
pub fn PlateFrame(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Accent(&self) -> &ColumnData {
&self.columns[6]
}
pub fn SortKey(&self) -> &ColumnData {
&self.columns[7]
}
pub fn TopBorder(&self) -> &ColumnData {
&self.columns[8]
}
pub fn BottomBorder(&self) -> &ColumnData {
&self.columns[9]
}
}

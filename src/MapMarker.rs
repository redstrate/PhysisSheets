#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MapMarker {
exd: EXD,
exh: EXH,
}
impl MapMarker {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MapMarker").unwrap();let exd = game_data.read_excel_sheet("MapMarker", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MapMarkerRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MapMarkerRow { columns: row.columns.clone() }
}
}
pub struct MapMarkerRow {
columns: Vec<ColumnData>,
}
impl MapMarkerRow {
pub fn Icon(&self) -> &ColumnData {
&self.columns[0]
}
pub fn PlaceNameSubtext(&self) -> &ColumnData {
&self.columns[1]
}
pub fn DataKey(&self) -> &ColumnData {
&self.columns[2]
}
pub fn X(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Y(&self) -> &ColumnData {
&self.columns[4]
}
pub fn SubtextOrientation(&self) -> &ColumnData {
&self.columns[5]
}
pub fn MapMarkerRegion(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Type(&self) -> &ColumnData {
&self.columns[7]
}
pub fn DataType(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[10]
}
}

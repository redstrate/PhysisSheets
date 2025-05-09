#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct MapMarkerSheet {
exd: EXD,
exh: EXH,
}
impl MapMarkerSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("MapMarker")?;let exd = game_data.read_excel_sheet("MapMarker", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<MapMarkerRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(MapMarkerRow { columns })
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

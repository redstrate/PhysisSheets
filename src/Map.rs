#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct Map {
exd: EXD,
exh: EXH,
}
impl Map {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("Map")?;let exd = game_data.read_excel_sheet("Map", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<MapRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(MapRow { columns })
}
}
pub struct MapRow {
columns: Vec<ColumnData>,
}
impl MapRow {
pub fn Id(&self) -> &ColumnData {
&self.columns[0]
}
pub fn DiscoveryFlag(&self) -> &ColumnData {
&self.columns[1]
}
pub fn MapMarkerRange(&self) -> &ColumnData {
&self.columns[2]
}
pub fn SizeFactor(&self) -> &ColumnData {
&self.columns[3]
}
pub fn PlaceNameRegion(&self) -> &ColumnData {
&self.columns[4]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[5]
}
pub fn PlaceNameSub(&self) -> &ColumnData {
&self.columns[6]
}
pub fn TerritoryType(&self) -> &ColumnData {
&self.columns[7]
}
pub fn OffsetX(&self) -> &ColumnData {
&self.columns[8]
}
pub fn OffsetY(&self) -> &ColumnData {
&self.columns[9]
}
pub fn DiscoveryIndex(&self) -> &ColumnData {
&self.columns[10]
}
pub fn MapCondition(&self) -> &ColumnData {
&self.columns[11]
}
pub fn PriorityCategoryUI(&self) -> &ColumnData {
&self.columns[12]
}
pub fn PriorityUI(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Hierarchy(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[16]
}
pub fn MapIndex(&self) -> &ColumnData {
&self.columns[17]
}
pub fn DiscoveryArrayByte(&self) -> &ColumnData {
&self.columns[18]
}
pub fn IsEvent(&self) -> &ColumnData {
&self.columns[19]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[20]
}
}

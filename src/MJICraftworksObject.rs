#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct MJICraftworksObjectSheet {
exd: EXD,
exh: EXH,
}
impl MJICraftworksObjectSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("MJICraftworksObject")?;let exd = game_data.read_excel_sheet("MJICraftworksObject", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<MJICraftworksObjectRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(MJICraftworksObjectRow { columns })
}
}
pub struct MJICraftworksObjectRow {
columns: Vec<ColumnData>,
}
impl MJICraftworksObjectRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Theme(&self) -> [&ColumnData; 2] {
[&self.columns[1],&self.columns[2],]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Material(&self) -> [&ColumnData; 4] {
[&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],]
}
pub fn Amount(&self) -> [&ColumnData; 4] {
[&self.columns[8],&self.columns[9],&self.columns[10],&self.columns[11],]
}
pub fn LevelReq(&self) -> &ColumnData {
&self.columns[12]
}
pub fn CraftingTime(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Value(&self) -> &ColumnData {
&self.columns[14]
}
}

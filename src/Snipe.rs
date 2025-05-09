#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct Snipe {
exd: EXD,
exh: EXH,
}
impl Snipe {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Snipe").unwrap();let exd = game_data.read_excel_sheet("Snipe", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> SnipeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
SnipeRow { columns }
}
}
pub struct SnipeRow {
columns: Vec<ColumnData>,
}
impl SnipeRow {
pub fn SnipeData(&self) -> &ColumnData {
&self.columns[0]
}
pub fn EventNPC(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Objective0(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Hint0(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Objective1(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Hint1(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[18]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[19]
}
pub fn Unknown14(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Unknown15(&self) -> &ColumnData {
&self.columns[21]
}
pub fn Unknown16(&self) -> &ColumnData {
&self.columns[22]
}
pub fn Unknown17(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Unknown18(&self) -> &ColumnData {
&self.columns[24]
}
pub fn ActionText(&self) -> &ColumnData {
&self.columns[25]
}
pub fn Unknown19(&self) -> &ColumnData {
&self.columns[26]
}
pub fn Unknown20(&self) -> &ColumnData {
&self.columns[27]
}
pub fn VFXFire(&self) -> &ColumnData {
&self.columns[28]
}
pub fn VFXHit(&self) -> &ColumnData {
&self.columns[29]
}
pub fn VFXMiss(&self) -> &ColumnData {
&self.columns[30]
}
pub fn VFXAdditional(&self) -> &ColumnData {
&self.columns[31]
}
pub fn LGBTargetMarker(&self) -> &ColumnData {
&self.columns[32]
}
pub fn Unknown21(&self) -> &ColumnData {
&self.columns[33]
}
pub fn Unknown22(&self) -> &ColumnData {
&self.columns[34]
}
pub fn Unknown23(&self) -> &ColumnData {
&self.columns[35]
}
pub fn Unknown24(&self) -> &ColumnData {
&self.columns[36]
}
pub fn Unknown25(&self) -> &ColumnData {
&self.columns[37]
}
pub fn Unknown26(&self) -> &ColumnData {
&self.columns[38]
}
pub fn Unknown27(&self) -> &ColumnData {
&self.columns[39]
}
pub fn Unknown28(&self) -> &ColumnData {
&self.columns[40]
}
pub fn Unknown29(&self) -> &ColumnData {
&self.columns[41]
}
pub fn Unknown30(&self) -> &ColumnData {
&self.columns[42]
}
pub fn Unknown31(&self) -> &ColumnData {
&self.columns[43]
}
pub fn Unknown32(&self) -> &ColumnData {
&self.columns[44]
}
}

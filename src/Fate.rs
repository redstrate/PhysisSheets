#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct Fate {
exd: EXD,
exh: EXH,
}
impl Fate {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Fate").unwrap();let exd = game_data.read_excel_sheet("Fate", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FateRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
FateRow { columns }
}
}
pub struct FateRow {
columns: Vec<ColumnData>,
}
impl FateRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Objective(&self) -> &ColumnData {
&self.columns[2]
}
pub fn StatusText(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[5]
}
pub fn ReqEventItem(&self) -> &ColumnData {
&self.columns[6]
}
pub fn TurnInEventItem(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[11]
}
pub fn ObjectiveIcon(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Location(&self) -> &ColumnData {
&self.columns[13]
}
pub fn EventItem(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[15]
}
pub fn MapIcon(&self) -> &ColumnData {
&self.columns[16]
}
pub fn InactiveMapIcon(&self) -> &ColumnData {
&self.columns[17]
}
pub fn LGBGuardNPCLocation(&self) -> &ColumnData {
&self.columns[18]
}
pub fn RequiredQuest(&self) -> &ColumnData {
&self.columns[19]
}
pub fn FATEChain(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[21]
}
pub fn ArrayIndex(&self) -> &ColumnData {
&self.columns[22]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[23]
}
pub fn FateRuleEx(&self) -> &ColumnData {
&self.columns[24]
}
pub fn Music(&self) -> &ColumnData {
&self.columns[25]
}
pub fn ScreenImageAccept(&self) -> &ColumnData {
&self.columns[26]
}
pub fn ScreenImageComplete(&self) -> &ColumnData {
&self.columns[27]
}
pub fn ScreenImageFailed(&self) -> &ColumnData {
&self.columns[28]
}
pub fn GivenStatus(&self) -> &ColumnData {
&self.columns[29]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[30]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[31]
}
pub fn EurekaFate(&self) -> &ColumnData {
&self.columns[32]
}
pub fn Rule(&self) -> &ColumnData {
&self.columns[33]
}
pub fn ClassJobLevel(&self) -> &ColumnData {
&self.columns[34]
}
pub fn ClassJobLevelMax(&self) -> &ColumnData {
&self.columns[35]
}
pub fn StatusValue(&self) -> &ColumnData {
&self.columns[36]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[37]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[38]
}
pub fn SpecialFate(&self) -> &ColumnData {
&self.columns[39]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[40]
}
pub fn AdventEvent(&self) -> &ColumnData {
&self.columns[41]
}
pub fn MoonFaireEvent(&self) -> &ColumnData {
&self.columns[42]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[43]
}
}

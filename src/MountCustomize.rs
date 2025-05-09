#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MountCustomize {
exd: EXD,
exh: EXH,
}
impl MountCustomize {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MountCustomize").unwrap();let exd = game_data.read_excel_sheet("MountCustomize", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MountCustomizeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MountCustomizeRow { columns: row.columns.clone() }
}
}
pub struct MountCustomizeRow {
columns: Vec<ColumnData>,
}
impl MountCustomizeRow {
pub fn HyurMidlanderMaleScale(&self) -> &ColumnData {
&self.columns[0]
}
pub fn HyurMidlanderFemaleScale(&self) -> &ColumnData {
&self.columns[1]
}
pub fn HyurHighlanderMaleScale(&self) -> &ColumnData {
&self.columns[2]
}
pub fn HyurHighlanderFemaleScale(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ElezenMaleScale(&self) -> &ColumnData {
&self.columns[4]
}
pub fn ElezenFemaleScale(&self) -> &ColumnData {
&self.columns[5]
}
pub fn LalaMaleScale(&self) -> &ColumnData {
&self.columns[6]
}
pub fn LalaFemaleScale(&self) -> &ColumnData {
&self.columns[7]
}
pub fn MiqoMaleScale(&self) -> &ColumnData {
&self.columns[8]
}
pub fn MiqoFemaleScale(&self) -> &ColumnData {
&self.columns[9]
}
pub fn RoeMaleScale(&self) -> &ColumnData {
&self.columns[10]
}
pub fn RoeFemaleScale(&self) -> &ColumnData {
&self.columns[11]
}
pub fn AuRaMaleScale(&self) -> &ColumnData {
&self.columns[12]
}
pub fn AuRaFemaleScale(&self) -> &ColumnData {
&self.columns[13]
}
pub fn HrothgarMaleScale(&self) -> &ColumnData {
&self.columns[14]
}
pub fn HrothgarFemaleScale(&self) -> &ColumnData {
&self.columns[15]
}
pub fn VieraMaleScale(&self) -> &ColumnData {
&self.columns[16]
}
pub fn VieraFemaleScale(&self) -> &ColumnData {
&self.columns[17]
}
pub fn HyurMidlanderMaleCameraHeight(&self) -> &ColumnData {
&self.columns[18]
}
pub fn HyurMidlanderFemaleCameraHeight(&self) -> &ColumnData {
&self.columns[19]
}
pub fn HyurHighlanderMaleCameraHeight(&self) -> &ColumnData {
&self.columns[20]
}
pub fn HyurHighlanderFemaleCameraHeight(&self) -> &ColumnData {
&self.columns[21]
}
pub fn ElezenMaleCameraHeight(&self) -> &ColumnData {
&self.columns[22]
}
pub fn ElezenFemaleCameraHeight(&self) -> &ColumnData {
&self.columns[23]
}
pub fn LalaMaleCameraHeight(&self) -> &ColumnData {
&self.columns[24]
}
pub fn LalaFemaleCameraHeight(&self) -> &ColumnData {
&self.columns[25]
}
pub fn MiqoMaleCameraHeight(&self) -> &ColumnData {
&self.columns[26]
}
pub fn MiqoFemaleCameraHeight(&self) -> &ColumnData {
&self.columns[27]
}
pub fn RoeMaleCameraHeight(&self) -> &ColumnData {
&self.columns[28]
}
pub fn RoeFemaleCameraHeight(&self) -> &ColumnData {
&self.columns[29]
}
pub fn AuRaMaleCameraHeight(&self) -> &ColumnData {
&self.columns[30]
}
pub fn AuRaFemaleCameraHeight(&self) -> &ColumnData {
&self.columns[31]
}
pub fn HrothgarMaleCameraHeight(&self) -> &ColumnData {
&self.columns[32]
}
pub fn HrothgarFemaleCameraHeight(&self) -> &ColumnData {
&self.columns[33]
}
pub fn VieraMaleCameraHeight(&self) -> &ColumnData {
&self.columns[34]
}
pub fn VieraFemaleCameraHeight(&self) -> &ColumnData {
&self.columns[35]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[36]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[37]
}
pub fn Unknown_70_1(&self) -> &ColumnData {
&self.columns[38]
}
pub fn Unknown_70_2(&self) -> &ColumnData {
&self.columns[39]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[40]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[41]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[42]
}
}

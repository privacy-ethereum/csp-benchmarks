#include <stdio.h>
#include <iostream>
#include <assert.h>
#include "circom.hpp"
#include "calcwit.hpp"
void ToBits_0_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void ToBits_0_run(uint ctx_index,Circom_CalcWit* ctx);
void UnpackBytes_1_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void UnpackBytes_1_run(uint ctx_index,Circom_CalcWit* ctx);
void StructureState_2_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void StructureState_2_run(uint ctx_index,Circom_CalcWit* ctx);
void LinearizeState_3_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void LinearizeState_3_run(uint ctx_index,Circom_CalcWit* ctx);
void ToBits_4_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void ToBits_4_run(uint ctx_index,Circom_CalcWit* ctx);
void XorN_5_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void XorN_5_run(uint ctx_index,Circom_CalcWit* ctx);
void Theta_6_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Theta_6_run(uint ctx_index,Circom_CalcWit* ctx);
void Rho_7_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Rho_7_run(uint ctx_index,Circom_CalcWit* ctx);
void Pi_8_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Pi_8_run(uint ctx_index,Circom_CalcWit* ctx);
void Chi_9_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Chi_9_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_10_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_10_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_11_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_11_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_12_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_12_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_13_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_13_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_14_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_14_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_15_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_15_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_16_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_16_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_17_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_17_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_18_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_18_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_19_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_19_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_20_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_20_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_21_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_21_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_22_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_22_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_23_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_23_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_24_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_24_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_25_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_25_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_26_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_26_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_27_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_27_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_28_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_28_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_29_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_29_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_30_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_30_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_31_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_31_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_32_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_32_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_33_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_33_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_34_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_34_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_35_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_35_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_36_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_36_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_37_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_37_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_38_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_38_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_39_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_39_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_40_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_40_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_41_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_41_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_42_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_42_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_43_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_43_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_44_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_44_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_45_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_45_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_46_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_46_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_47_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_47_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_48_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_48_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_49_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_49_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_50_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_50_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_51_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_51_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_52_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_52_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_53_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_53_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_54_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_54_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_55_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_55_run(uint ctx_index,Circom_CalcWit* ctx);
void Iota_56_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Iota_56_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakStep_57_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakStep_57_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakP_58_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakP_58_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakF_59_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakF_59_run(uint ctx_index,Circom_CalcWit* ctx);
void LinearizedKeccakF_60_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void LinearizedKeccakF_60_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakSponge_61_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakSponge_61_run(uint ctx_index,Circom_CalcWit* ctx);
void Keccak_256_62_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Keccak_256_62_run(uint ctx_index,Circom_CalcWit* ctx);
void PackBytes_63_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void PackBytes_63_run(uint ctx_index,Circom_CalcWit* ctx);
void Keccak_256_bytes_64_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Keccak_256_bytes_64_run(uint ctx_index,Circom_CalcWit* ctx);
void Keccak256Hash_65_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Keccak256Hash_65_run(uint ctx_index,Circom_CalcWit* ctx);
void CalcSmallRC_0(Circom_CalcWit* ctx,FrElement* lvar,uint componentFather,FrElement* destination,int destination_size);
Circom_TemplateFunction _functionTable[66] = { 
ToBits_0_run,
UnpackBytes_1_run,
StructureState_2_run,
LinearizeState_3_run,
ToBits_4_run,
XorN_5_run,
Theta_6_run,
Rho_7_run,
Pi_8_run,
Chi_9_run,
Iota_10_run,
KeccakStep_11_run,
Iota_12_run,
KeccakStep_13_run,
Iota_14_run,
KeccakStep_15_run,
Iota_16_run,
KeccakStep_17_run,
Iota_18_run,
KeccakStep_19_run,
Iota_20_run,
KeccakStep_21_run,
Iota_22_run,
KeccakStep_23_run,
Iota_24_run,
KeccakStep_25_run,
Iota_26_run,
KeccakStep_27_run,
Iota_28_run,
KeccakStep_29_run,
Iota_30_run,
KeccakStep_31_run,
Iota_32_run,
KeccakStep_33_run,
Iota_34_run,
KeccakStep_35_run,
Iota_36_run,
KeccakStep_37_run,
Iota_38_run,
KeccakStep_39_run,
Iota_40_run,
KeccakStep_41_run,
Iota_42_run,
KeccakStep_43_run,
Iota_44_run,
KeccakStep_45_run,
Iota_46_run,
KeccakStep_47_run,
Iota_48_run,
KeccakStep_49_run,
Iota_50_run,
KeccakStep_51_run,
Iota_52_run,
KeccakStep_53_run,
Iota_54_run,
KeccakStep_55_run,
Iota_56_run,
KeccakStep_57_run,
KeccakP_58_run,
KeccakF_59_run,
LinearizedKeccakF_60_run,
KeccakSponge_61_run,
Keccak_256_62_run,
PackBytes_63_run,
Keccak_256_bytes_64_run,
Keccak256Hash_65_run };
Circom_TemplateFunction _functionTableParallel[66] = { 
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL };
uint get_main_input_signal_start() {return 33;}

uint get_main_input_signal_no() {return 544;}

uint get_total_signal_no() {return 2625473;}

uint get_number_of_components() {return 62554;}

uint get_size_of_input_hashmap() {return 256;}

uint get_size_of_witness() {return 623737;}

uint get_size_of_constants() {return 89;}

uint get_size_of_io_map() {return 24;}

uint get_size_of_bus_field_map() {return 0;}

void release_memory_component(Circom_CalcWit* ctx, uint pos) {{

if (pos != 0){{

if(ctx->componentMemory[pos].subcomponents)
delete []ctx->componentMemory[pos].subcomponents;

if(ctx->componentMemory[pos].subcomponentsParallel)
delete []ctx->componentMemory[pos].subcomponentsParallel;

if(ctx->componentMemory[pos].outputIsSet)
delete []ctx->componentMemory[pos].outputIsSet;

if(ctx->componentMemory[pos].mutexes)
delete []ctx->componentMemory[pos].mutexes;

if(ctx->componentMemory[pos].cvs)
delete []ctx->componentMemory[pos].cvs;

if(ctx->componentMemory[pos].sbct)
delete []ctx->componentMemory[pos].sbct;

}}


}}


// function declarations
void CalcSmallRC_0(Circom_CalcWit* ctx,FrElement* lvar,uint componentFather,FrElement* destination,int destination_size){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement expaux[3];
std::string myTemplateName = "CalcSmallRC";
u64 myId = componentFather;
{
PFrElement aux_dest = &lvar[1];
// load src
Fr_mod(&expaux[0],&lvar[0],&circuitConstants[87]); // line circom 62
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{{
Fr_eq(&expaux[0],&lvar[1],&circuitConstants[1]); // line circom 63
}}
if(Fr_isTrue(&expaux[0])){
// return bucket
Fr_copy(destination,&circuitConstants[2]);
return;
}else{
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[2]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[2]);
}
Fr_leq(&expaux[0],&lvar[3],&lvar[1]); // line circom 68
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_shl(&expaux[0],&lvar[2],&circuitConstants[2]); // line circom 69
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
Fr_shr(&expaux[1],&lvar[2],&circuitConstants[0]); // line circom 70
Fr_band(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 70
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_mul(&expaux[1],&lvar[4],&circuitConstants[88]); // line circom 71
Fr_bxor(&expaux[0],&lvar[2],&expaux[1]); // line circom 71
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_band(&expaux[0],&lvar[2],&circuitConstants[87]); // line circom 72
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
Fr_add(&expaux[0],&lvar[3],&circuitConstants[2]); // line circom 68
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[3],&lvar[1]); // line circom 68
}
// return bucket
Fr_band(&expaux[0],&lvar[2],&circuitConstants[2]); // line circom 74
Fr_copy(destination,&expaux[0]);
return;
}
}

// template declarations
void ToBits_0_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 0;
ctx->componentMemory[coffset].templateName = "ToBits";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void ToBits_0_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[0]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[0]); // line circom 22
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[2])) + 0)];
// load src
Fr_shr(&expaux[1],&signalValues[mySignalStart + 8],&lvar[2]); // line circom 23
Fr_band(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 23
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
Fr_sub(&expaux[2],&circuitConstants[2],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[2])) + 0)]); // line circom 24
Fr_mul(&expaux[1],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[2])) + 0)],&expaux[2]); // line circom 24
{{
Fr_eq(&expaux[0],&expaux[1],&circuitConstants[1]); // line circom 24
}}
if (!Fr_isTrue(&expaux[0])) std::cout << "Failed assert in template/function " << myTemplateName << " line 24. " <<  "Followed trace of components: " << ctx->getTrace(myId) << std::endl;
assert(Fr_isTrue(&expaux[0]));
}
{
PFrElement aux_dest = &lvar[1];
// load src
Fr_shl(&expaux[2],&circuitConstants[2],&lvar[2]); // line circom 25
Fr_mul(&expaux[1],&expaux[2],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[2])) + 0)]); // line circom 25
Fr_add(&expaux[0],&lvar[1],&expaux[1]); // line circom 25
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_add(&expaux[0],&lvar[2],&circuitConstants[2]); // line circom 22
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[0]); // line circom 22
}
{
{{
Fr_eq(&expaux[0],&signalValues[mySignalStart + 8],&lvar[1]); // line circom 28
}}
if (!Fr_isTrue(&expaux[0])) std::cout << "Failed assert in template/function " << myTemplateName << " line 28. " <<  "Followed trace of components: " << ctx->getTrace(myId) << std::endl;
assert(Fr_isTrue(&expaux[0]));
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void UnpackBytes_1_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 1;
ctx->componentMemory[coffset].templateName = "UnpackBytes";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 512;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[512]{0};
}

void UnpackBytes_1_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[2];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[3]);
}
{
uint aux_create = 0;
int aux_cmp_num = 0+ctx_index+1;
uint csoffset = mySignalStart+4608;
uint aux_dimensions[1] = {512};
for (uint i = 0; i < 512; i++) {
std::string new_cmp_name = "tobits"+ctx->generate_position_array(aux_dimensions, 1, i);
ToBits_0_create(csoffset,aux_cmp_num,ctx,new_cmp_name,myId);
mySubcomponents[aux_create+ i] = aux_cmp_num;
csoffset += 9 ;
aux_cmp_num += 1;
}
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[1],&circuitConstants[3]); // line circom 13
while(Fr_isTrue(&expaux[0])){
{
uint cmp_index_ref = ((1 * Fr_toInt(&lvar[1])) + 0);
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 8];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 4096)]);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1)){
ToBits_0_run(mySubcomponents[cmp_index_ref],ctx);

}
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[0]); // line circom 16
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1 * ((Fr_toInt(&lvar[1]) * 8) + Fr_toInt(&lvar[2]))) + 0)];
// load src
cmp_index_ref_load = ((1 * Fr_toInt(&lvar[1])) + 0);
cmp_index_ref_load = ((1 * Fr_toInt(&lvar[1])) + 0);
// end load src
Fr_copy(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[((1 * Fr_toInt(&lvar[1])) + 0)]].signalStart + ((1 * Fr_toInt(&lvar[2])) + 0)]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_add(&expaux[0],&lvar[2],&circuitConstants[2]); // line circom 16
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[0]); // line circom 16
}
{
PFrElement aux_dest = &lvar[1];
// load src
Fr_add(&expaux[0],&lvar[1],&circuitConstants[2]); // line circom 13
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[1],&circuitConstants[3]); // line circom 13
}
for (uint i = 0; i < 512; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void StructureState_2_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 2;
ctx->componentMemory[coffset].templateName = "StructureState";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void StructureState_2_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[2];
FrElement lvar[6];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[6]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[7]); // line circom 153
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[4],&circuitConstants[7]); // line circom 154
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[5],&circuitConstants[5]); // line circom 155
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[3])) + (64 * Fr_toInt(&lvar[4]))) + (1 * Fr_toInt(&lvar[5]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((1 * ((64 * ((5 * Fr_toInt(&lvar[4])) + Fr_toInt(&lvar[3]))) + Fr_toInt(&lvar[5]))) + 1600)]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
Fr_add(&expaux[0],&lvar[5],&circuitConstants[2]); // line circom 155
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[5],&circuitConstants[5]); // line circom 155
}
{
PFrElement aux_dest = &lvar[4];
// load src
Fr_add(&expaux[0],&lvar[4],&circuitConstants[2]); // line circom 154
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[4],&circuitConstants[7]); // line circom 154
}
{
PFrElement aux_dest = &lvar[3];
// load src
Fr_add(&expaux[0],&lvar[3],&circuitConstants[2]); // line circom 153
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[7]); // line circom 153
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void LinearizeState_3_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 3;
ctx->componentMemory[coffset].templateName = "LinearizeState";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void LinearizeState_3_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[2];
FrElement lvar[6];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[6]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[7]); // line circom 138
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[4],&circuitConstants[7]); // line circom 139
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[5],&circuitConstants[5]); // line circom 140
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1 * ((64 * ((5 * Fr_toInt(&lvar[4])) + Fr_toInt(&lvar[3]))) + Fr_toInt(&lvar[5]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[3])) + (64 * Fr_toInt(&lvar[4]))) + (1 * Fr_toInt(&lvar[5]))) + 1600)]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
Fr_add(&expaux[0],&lvar[5],&circuitConstants[2]); // line circom 140
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[5],&circuitConstants[5]); // line circom 140
}
{
PFrElement aux_dest = &lvar[4];
// load src
Fr_add(&expaux[0],&lvar[4],&circuitConstants[2]); // line circom 139
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[4],&circuitConstants[7]); // line circom 139
}
{
PFrElement aux_dest = &lvar[3];
// load src
Fr_add(&expaux[0],&lvar[3],&circuitConstants[2]); // line circom 138
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[7]); // line circom 138
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void ToBits_4_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 4;
ctx->componentMemory[coffset].templateName = "ToBits";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void ToBits_4_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[8]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[8]); // line circom 22
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[2])) + 0)];
// load src
Fr_shr(&expaux[1],&signalValues[mySignalStart + 3],&lvar[2]); // line circom 23
Fr_band(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 23
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
Fr_sub(&expaux[2],&circuitConstants[2],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[2])) + 0)]); // line circom 24
Fr_mul(&expaux[1],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[2])) + 0)],&expaux[2]); // line circom 24
{{
Fr_eq(&expaux[0],&expaux[1],&circuitConstants[1]); // line circom 24
}}
if (!Fr_isTrue(&expaux[0])) std::cout << "Failed assert in template/function " << myTemplateName << " line 24. " <<  "Followed trace of components: " << ctx->getTrace(myId) << std::endl;
assert(Fr_isTrue(&expaux[0]));
}
{
PFrElement aux_dest = &lvar[1];
// load src
Fr_shl(&expaux[2],&circuitConstants[2],&lvar[2]); // line circom 25
Fr_mul(&expaux[1],&expaux[2],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[2])) + 0)]); // line circom 25
Fr_add(&expaux[0],&lvar[1],&expaux[1]); // line circom 25
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_add(&expaux[0],&lvar[2],&circuitConstants[2]); // line circom 22
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[8]); // line circom 22
}
{
{{
Fr_eq(&expaux[0],&signalValues[mySignalStart + 3],&lvar[1]); // line circom 28
}}
if (!Fr_isTrue(&expaux[0])) std::cout << "Failed assert in template/function " << myTemplateName << " line 28. " <<  "Followed trace of components: " << ctx->getTrace(myId) << std::endl;
assert(Fr_isTrue(&expaux[0]));
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void XorN_5_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 5;
ctx->componentMemory[coffset].templateName = "XorN";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 5;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[1]{0};
}

void XorN_5_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[2];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[7]);
}
{
std::string new_cmp_name = "tb";
ToBits_4_create(mySignalStart+6,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 0+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[7]); // line circom 47
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[1];
// load src
Fr_add(&expaux[0],&lvar[1],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[2])) + 1)]); // line circom 48
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_add(&expaux[0],&lvar[2],&circuitConstants[2]); // line circom 47
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[7]); // line circom 47
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[8]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 3];
// load src
// end load src
Fr_copy(aux_dest,&lvar[1]);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
ToBits_4_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copy(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0]);
}
for (uint i = 0; i < 1; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Theta_6_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 6;
ctx->componentMemory[coffset].templateName = "Theta";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[320]{0};
}

void Theta_6_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[5];
FrElement lvar[7];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
uint aux_create = 0;
int aux_cmp_num = 0+ctx_index+1;
uint csoffset = mySignalStart+3840;
uint aux_dimensions[2] = {5,64};
for (uint i = 0; i < 320; i++) {
std::string new_cmp_name = "xor"+ctx->generate_position_array(aux_dimensions, 2, i);
XorN_5_create(csoffset,aux_cmp_num,ctx,new_cmp_name,myId);
mySubcomponents[aux_create+ i] = aux_cmp_num;
csoffset += 10 ;
aux_cmp_num += 2;
}
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[7]); // line circom 175
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[5]); // line circom 176
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[4],&circuitConstants[7]); // line circom 178
while(Fr_isTrue(&expaux[0])){
{
uint cmp_index_ref = (((64 * Fr_toInt(&lvar[2])) + (1 * Fr_toInt(&lvar[3]))) + 0);
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + ((1 * Fr_toInt(&lvar[4])) + 1)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[2])) + (64 * Fr_toInt(&lvar[4]))) + (1 * Fr_toInt(&lvar[3]))) + 1600)]);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1)){
XorN_5_run(mySubcomponents[cmp_index_ref],ctx);

}
}
{
PFrElement aux_dest = &lvar[4];
// load src
Fr_add(&expaux[0],&lvar[4],&circuitConstants[2]); // line circom 178
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[4],&circuitConstants[7]); // line circom 178
}
{
PFrElement aux_dest = &signalValues[mySignalStart + (((64 * Fr_toInt(&lvar[2])) + (1 * Fr_toInt(&lvar[3]))) + 3200)];
// load src
cmp_index_ref_load = (((64 * Fr_toInt(&lvar[2])) + (1 * Fr_toInt(&lvar[3]))) + 0);
cmp_index_ref_load = (((64 * Fr_toInt(&lvar[2])) + (1 * Fr_toInt(&lvar[3]))) + 0);
// end load src
Fr_copy(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[(((64 * Fr_toInt(&lvar[2])) + (1 * Fr_toInt(&lvar[3]))) + 0)]].signalStart + 0]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
Fr_add(&expaux[0],&lvar[3],&circuitConstants[2]); // line circom 176
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[5]); // line circom 176
}
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_add(&expaux[0],&lvar[2],&circuitConstants[2]); // line circom 175
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[7]); // line circom 175
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[7]); // line circom 183
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[5]); // line circom 184
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[4];
// load src
Fr_add(&expaux[1],&lvar[2],&circuitConstants[9]); // line circom 185
Fr_mod(&expaux[0],&expaux[1],&circuitConstants[7]); // line circom 185
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + (((64 * Fr_toInt(&expaux[0])) + (1 * Fr_toInt(&lvar[3]))) + 3200)]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
Fr_add(&expaux[1],&lvar[2],&circuitConstants[2]); // line circom 186
Fr_mod(&expaux[0],&expaux[1],&circuitConstants[7]); // line circom 186
Fr_add(&expaux[3],&lvar[3],&circuitConstants[5]); // line circom 186
Fr_sub(&expaux[2],&expaux[3],&circuitConstants[2]); // line circom 186
Fr_mod(&expaux[1],&expaux[2],&circuitConstants[5]); // line circom 186
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + (((64 * Fr_toInt(&expaux[0])) + (1 * Fr_toInt(&expaux[1]))) + 3200)]);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + (((64 * Fr_toInt(&lvar[2])) + (1 * Fr_toInt(&lvar[3]))) + 3520)];
// load src
Fr_add(&expaux[1],&lvar[4],&lvar[5]); // line circom 187
Fr_mul(&expaux[3],&circuitConstants[10],&lvar[4]); // line circom 187
Fr_mul(&expaux[2],&expaux[3],&lvar[5]); // line circom 187
Fr_sub(&expaux[0],&expaux[1],&expaux[2]); // line circom 187
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
Fr_add(&expaux[0],&lvar[3],&circuitConstants[2]); // line circom 184
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[5]); // line circom 184
}
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_add(&expaux[0],&lvar[2],&circuitConstants[2]); // line circom 183
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[7]); // line circom 183
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[7]); // line circom 191
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[5]); // line circom 192
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + (((64 * Fr_toInt(&lvar[2])) + (1 * Fr_toInt(&lvar[3]))) + 3520)]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[5],&circuitConstants[7]); // line circom 194
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[2])) + (64 * Fr_toInt(&lvar[5]))) + (1 * Fr_toInt(&lvar[3]))) + 1600)]);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[2])) + (64 * Fr_toInt(&lvar[5]))) + (1 * Fr_toInt(&lvar[3]))) + 0)];
// load src
Fr_add(&expaux[1],&lvar[6],&lvar[4]); // line circom 196
Fr_mul(&expaux[3],&circuitConstants[10],&lvar[6]); // line circom 196
Fr_mul(&expaux[2],&expaux[3],&lvar[4]); // line circom 196
Fr_sub(&expaux[0],&expaux[1],&expaux[2]); // line circom 196
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
Fr_add(&expaux[0],&lvar[5],&circuitConstants[2]); // line circom 194
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[5],&circuitConstants[7]); // line circom 194
}
{
PFrElement aux_dest = &lvar[3];
// load src
Fr_add(&expaux[0],&lvar[3],&circuitConstants[2]); // line circom 192
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[5]); // line circom 192
}
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_add(&expaux[0],&lvar[2],&circuitConstants[2]); // line circom 191
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[7]); // line circom 191
}
for (uint i = 0; i < 320; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Rho_7_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 7;
ctx->componentMemory[coffset].templateName = "Rho";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Rho_7_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[6];
FrElement lvar[9];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[5]); // line circom 210
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[2]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[2]))) + 1600)]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_add(&expaux[0],&lvar[2],&circuitConstants[2]); // line circom 210
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[5]); // line circom 210
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[2]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[4],&circuitConstants[11]); // line circom 214
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[5];
// load src
Fr_add(&expaux[2],&lvar[4],&circuitConstants[2]); // line circom 215
Fr_add(&expaux[3],&lvar[4],&circuitConstants[10]); // line circom 215
Fr_mul(&expaux[1],&expaux[2],&expaux[3]); // line circom 215
Fr_idiv(&expaux[0],&expaux[1],&circuitConstants[10]); // line circom 215
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
Fr_neg(&expaux[1],&lvar[5]); // line circom 216
Fr_idiv(&expaux[4],&lvar[5],&circuitConstants[5]); // line circom 216
Fr_add(&expaux[3],&expaux[4],&circuitConstants[2]); // line circom 216
Fr_mul(&expaux[2],&expaux[3],&circuitConstants[5]); // line circom 216
Fr_add(&expaux[0],&expaux[1],&expaux[2]); // line circom 216
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
Fr_mod(&expaux[0],&lvar[6],&circuitConstants[5]); // line circom 217
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[8],&circuitConstants[5]); // line circom 219
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[2])) + (64 * Fr_toInt(&lvar[3]))) + (1 * Fr_toInt(&lvar[8]))) + 0)];
// load src
Fr_add(&expaux[1],&lvar[8],&lvar[7]); // line circom 219
Fr_mod(&expaux[0],&expaux[1],&circuitConstants[5]); // line circom 219
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[2])) + (64 * Fr_toInt(&lvar[3]))) + (1 * Fr_toInt(&expaux[0]))) + 1600)]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
Fr_add(&expaux[0],&lvar[8],&circuitConstants[2]); // line circom 219
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[8],&circuitConstants[5]); // line circom 219
}
{
PFrElement aux_dest = &lvar[8];
// load src
Fr_mul(&expaux[2],&circuitConstants[10],&lvar[2]); // line circom 221
Fr_mul(&expaux[3],&circuitConstants[8],&lvar[3]); // line circom 221
Fr_add(&expaux[1],&expaux[2],&expaux[3]); // line circom 221
Fr_mod(&expaux[0],&expaux[1],&circuitConstants[7]); // line circom 221
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&lvar[3]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&lvar[8]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
Fr_add(&expaux[0],&lvar[4],&circuitConstants[2]); // line circom 214
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[4],&circuitConstants[11]); // line circom 214
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Pi_8_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 8;
ctx->componentMemory[coffset].templateName = "Pi";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Pi_8_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[5];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[7]); // line circom 236
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[7]); // line circom 237
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[4],&circuitConstants[5]); // line circom 238
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[2])) + (64 * Fr_toInt(&lvar[3]))) + (1 * Fr_toInt(&lvar[4]))) + 0)];
// load src
Fr_mul(&expaux[2],&circuitConstants[8],&lvar[3]); // line circom 239
Fr_add(&expaux[1],&lvar[2],&expaux[2]); // line circom 239
Fr_mod(&expaux[0],&expaux[1],&circuitConstants[7]); // line circom 239
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&expaux[0])) + (64 * Fr_toInt(&lvar[2]))) + (1 * Fr_toInt(&lvar[4]))) + 1600)]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
Fr_add(&expaux[0],&lvar[4],&circuitConstants[2]); // line circom 238
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[4],&circuitConstants[5]); // line circom 238
}
{
PFrElement aux_dest = &lvar[3];
// load src
Fr_add(&expaux[0],&lvar[3],&circuitConstants[2]); // line circom 237
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[7]); // line circom 237
}
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_add(&expaux[0],&lvar[2],&circuitConstants[2]); // line circom 236
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[7]); // line circom 236
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Chi_9_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 9;
ctx->componentMemory[coffset].templateName = "Chi";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Chi_9_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[5];
FrElement lvar[8];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[7]); // line circom 255
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[7]); // line circom 256
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[4],&circuitConstants[5]); // line circom 257
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[2])) + (64 * Fr_toInt(&lvar[3]))) + (1 * Fr_toInt(&lvar[4]))) + 1600)]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
Fr_add(&expaux[1],&lvar[2],&circuitConstants[2]); // line circom 259
Fr_mod(&expaux[0],&expaux[1],&circuitConstants[7]); // line circom 259
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&expaux[0])) + (64 * Fr_toInt(&lvar[3]))) + (1 * Fr_toInt(&lvar[4]))) + 1600)]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
Fr_add(&expaux[1],&lvar[2],&circuitConstants[10]); // line circom 260
Fr_mod(&expaux[0],&expaux[1],&circuitConstants[7]); // line circom 260
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&expaux[0])) + (64 * Fr_toInt(&lvar[3]))) + (1 * Fr_toInt(&lvar[4]))) + 1600)]);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[2])) + (64 * Fr_toInt(&lvar[3]))) + (1 * Fr_toInt(&lvar[4]))) + 3200)];
// load src
Fr_sub(&expaux[1],&circuitConstants[2],&lvar[6]); // line circom 261
Fr_mul(&expaux[0],&expaux[1],&lvar[7]); // line circom 261
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[2])) + (64 * Fr_toInt(&lvar[3]))) + (1 * Fr_toInt(&lvar[4]))) + 0)];
// load src
Fr_add(&expaux[1],&lvar[5],&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[2])) + (64 * Fr_toInt(&lvar[3]))) + (1 * Fr_toInt(&lvar[4]))) + 3200)]); // line circom 262
Fr_mul(&expaux[3],&circuitConstants[10],&lvar[5]); // line circom 262
Fr_mul(&expaux[2],&expaux[3],&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[2])) + (64 * Fr_toInt(&lvar[3]))) + (1 * Fr_toInt(&lvar[4]))) + 3200)]); // line circom 262
Fr_sub(&expaux[0],&expaux[1],&expaux[2]); // line circom 262
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
Fr_add(&expaux[0],&lvar[4],&circuitConstants[2]); // line circom 257
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[4],&circuitConstants[5]); // line circom 257
}
{
PFrElement aux_dest = &lvar[3];
// load src
Fr_add(&expaux[0],&lvar[3],&circuitConstants[2]); // line circom 256
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[7]); // line circom 256
}
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_add(&expaux[0],&lvar[2],&circuitConstants[2]); // line circom 255
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[7]); // line circom 255
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_10_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 10;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_10_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + 1600]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_11_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 11;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_11_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_10_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_10_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_12_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 12;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_12_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[2]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[12]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_13_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 13;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_13_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[2]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_12_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_12_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_14_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 14;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_14_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[10]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[18]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_15_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 15;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_15_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[10]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_14_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_14_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_16_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 16;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_16_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[8]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[25]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_17_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 17;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_17_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[8]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_16_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_16_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_18_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 18;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_18_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[9]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[31]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_19_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 19;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_19_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[9]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_18_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_18_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_20_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 20;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_20_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[7]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[38]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_21_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 21;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_21_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[7]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_20_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_20_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_22_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 22;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_22_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[45]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_23_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 23;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_23_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_22_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_22_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_24_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 24;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_24_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[12]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[52]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_25_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 25;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_25_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[12]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_24_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_24_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_26_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 26;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_26_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[0]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[59]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_27_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 27;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_27_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[0]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_26_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_26_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_28_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 28;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_28_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[13]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[66]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_29_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 29;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_29_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[13]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_28_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_28_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_30_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 30;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_30_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[14]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[67]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_31_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 31;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_31_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[14]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_30_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_30_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_32_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 32;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_32_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[15]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[68]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_33_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 33;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_33_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[15]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_32_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_32_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_34_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 34;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_34_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[16]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[69]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_35_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 35;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_35_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[16]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_34_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_34_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_36_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 36;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_36_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[17]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[70]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_37_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 37;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_37_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[17]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_36_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_36_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_38_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 38;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_38_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[18]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[71]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_39_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 39;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_39_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[18]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_38_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_38_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_40_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 40;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_40_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[19]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[72]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_41_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 41;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_41_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[19]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_40_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_40_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_42_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 42;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_42_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[20]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[73]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_43_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 43;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_43_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[20]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_42_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_42_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_44_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 44;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_44_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[21]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[74]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_45_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 45;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_45_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[21]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_44_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_44_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_46_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 46;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_46_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[22]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[75]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_47_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 47;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_47_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[22]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_46_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_46_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_48_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 48;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_48_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[23]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[76]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_49_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 49;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_49_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[23]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_48_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_48_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_50_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 50;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_50_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[24]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[77]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_51_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 51;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_51_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[24]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_50_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_50_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_52_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 52;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_52_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[25]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[78]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_53_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 53;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_53_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[25]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_52_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_52_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_54_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 54;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_54_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[26]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[79]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_55_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 55;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_55_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[26]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_54_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_54_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Iota_56_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 56;
ctx->componentMemory[coffset].templateName = "Iota";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Iota_56_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[70];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[27]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[53];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[54];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[55];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[56];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[57];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[58];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[59];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[60];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[61];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[62];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[63];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[64];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[65];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[66];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[((1 * Fr_toInt(&lvar[67])) + 3)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 277
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[5]); // line circom 277
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
while(Fr_isTrue(&expaux[0])){
{

// start of call bucket
FrElement lvarcall[5];
// copying argument 0
Fr_add(&expaux[1],&lvar[67],&circuitConstants[80]); // line circom 281
Fr_copy(&lvarcall[0],&expaux[1]);
// end copying argument 0
Fr_shl(&expaux[1],&circuitConstants[2],&lvar[67]); // line circom 281
Fr_sub(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 281
CalcSmallRC_0(ctx,lvarcall,myId,&lvar[((1 * Fr_toInt(&expaux[0])) + 3)],1);
// end call bucket
}

{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 280
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_leq(&expaux[0],&lvar[67],&circuitConstants[4]); // line circom 280
}
{
PFrElement aux_dest = &lvar[67];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[68];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[69];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[67],&circuitConstants[1]); // line circom 294
}}
{{
Fr_eq(&expaux[2],&lvar[68],&circuitConstants[1]); // line circom 294
}}
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 294
if(Fr_isTrue(&expaux[0])){
Fr_neq(&expaux[0],&lvar[((1 * Fr_toInt(&lvar[69])) + 3)],&circuitConstants[1]); // line circom 295
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
Fr_sub(&expaux[0],&circuitConstants[2],&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]); // line circom 296
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((((320 * Fr_toInt(&lvar[67])) + (64 * Fr_toInt(&lvar[68]))) + (1 * Fr_toInt(&lvar[69]))) + 1600)]);
}
}
{
PFrElement aux_dest = &lvar[69];
// load src
Fr_add(&expaux[0],&lvar[69],&circuitConstants[2]); // line circom 292
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[69],&circuitConstants[5]); // line circom 292
}
{
PFrElement aux_dest = &lvar[68];
// load src
Fr_add(&expaux[0],&lvar[68],&circuitConstants[2]); // line circom 291
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[68],&circuitConstants[7]); // line circom 291
}
{
PFrElement aux_dest = &lvar[67];
// load src
Fr_add(&expaux[0],&lvar[67],&circuitConstants[2]); // line circom 290
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[67],&circuitConstants[7]); // line circom 290
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakStep_57_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 57;
ctx->componentMemory[coffset].templateName = "KeccakStep";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[5]{0};
}

void KeccakStep_57_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[27]);
}
{
std::string new_cmp_name = "theta";
Theta_6_create(mySignalStart+17600,4+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 4+ctx_index+1;
}
{
std::string new_cmp_name = "rho";
Rho_7_create(mySignalStart+14400,3+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 3+ctx_index+1;
}
{
std::string new_cmp_name = "pi";
Pi_8_create(mySignalStart+11200,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
std::string new_cmp_name = "chi";
Chi_9_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "iota";
Iota_56_create(mySignalStart+8000,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 1+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Theta_6_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Rho_7_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Pi_8_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 3;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Chi_9_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 4;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 3;
cmp_index_ref_load = 3;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[3]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Iota_56_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 4;
cmp_index_ref_load = 4;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[4]].signalStart + 0],1600);
}
for (uint i = 0; i < 5; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakP_58_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 58;
ctx->componentMemory[coffset].templateName = "KeccakP";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[24]{0};
}

void KeccakP_58_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[2];
FrElement lvar[4];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[11]);
}
{
std::string new_cmp_name = "steps[0]";
KeccakStep_11_create(mySignalStart+43200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "steps[1]";
KeccakStep_13_create(mySignalStart+67840,646+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 646+ctx_index+1;
}
{
std::string new_cmp_name = "steps[2]";
KeccakStep_15_create(mySignalStart+92480,1292+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 1292+ctx_index+1;
}
{
std::string new_cmp_name = "steps[3]";
KeccakStep_17_create(mySignalStart+117120,1938+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[3] = 1938+ctx_index+1;
}
{
std::string new_cmp_name = "steps[4]";
KeccakStep_19_create(mySignalStart+141760,2584+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[4] = 2584+ctx_index+1;
}
{
std::string new_cmp_name = "steps[5]";
KeccakStep_21_create(mySignalStart+166400,3230+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[5] = 3230+ctx_index+1;
}
{
std::string new_cmp_name = "steps[6]";
KeccakStep_23_create(mySignalStart+191040,3876+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[6] = 3876+ctx_index+1;
}
{
std::string new_cmp_name = "steps[7]";
KeccakStep_25_create(mySignalStart+215680,4522+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[7] = 4522+ctx_index+1;
}
{
std::string new_cmp_name = "steps[8]";
KeccakStep_27_create(mySignalStart+240320,5168+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[8] = 5168+ctx_index+1;
}
{
std::string new_cmp_name = "steps[9]";
KeccakStep_29_create(mySignalStart+264960,5814+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[9] = 5814+ctx_index+1;
}
{
std::string new_cmp_name = "steps[10]";
KeccakStep_31_create(mySignalStart+289600,6460+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[10] = 6460+ctx_index+1;
}
{
std::string new_cmp_name = "steps[11]";
KeccakStep_33_create(mySignalStart+314240,7106+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[11] = 7106+ctx_index+1;
}
{
std::string new_cmp_name = "steps[12]";
KeccakStep_35_create(mySignalStart+338880,7752+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[12] = 7752+ctx_index+1;
}
{
std::string new_cmp_name = "steps[13]";
KeccakStep_37_create(mySignalStart+363520,8398+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[13] = 8398+ctx_index+1;
}
{
std::string new_cmp_name = "steps[14]";
KeccakStep_39_create(mySignalStart+388160,9044+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[14] = 9044+ctx_index+1;
}
{
std::string new_cmp_name = "steps[15]";
KeccakStep_41_create(mySignalStart+412800,9690+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[15] = 9690+ctx_index+1;
}
{
std::string new_cmp_name = "steps[16]";
KeccakStep_43_create(mySignalStart+437440,10336+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[16] = 10336+ctx_index+1;
}
{
std::string new_cmp_name = "steps[17]";
KeccakStep_45_create(mySignalStart+462080,10982+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[17] = 10982+ctx_index+1;
}
{
std::string new_cmp_name = "steps[18]";
KeccakStep_47_create(mySignalStart+486720,11628+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[18] = 11628+ctx_index+1;
}
{
std::string new_cmp_name = "steps[19]";
KeccakStep_49_create(mySignalStart+511360,12274+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[19] = 12274+ctx_index+1;
}
{
std::string new_cmp_name = "steps[20]";
KeccakStep_51_create(mySignalStart+536000,12920+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[20] = 12920+ctx_index+1;
}
{
std::string new_cmp_name = "steps[21]";
KeccakStep_53_create(mySignalStart+560640,13566+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[21] = 13566+ctx_index+1;
}
{
std::string new_cmp_name = "steps[22]";
KeccakStep_55_create(mySignalStart+585280,14212+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[22] = 14212+ctx_index+1;
}
{
std::string new_cmp_name = "steps[23]";
KeccakStep_57_create(mySignalStart+609920,14858+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[23] = 14858+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 3200];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[11]); // line circom 347
while(Fr_isTrue(&expaux[0])){
{
uint cmp_index_ref = ((1 * Fr_toInt(&lvar[3])) + 0);
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + ctx->templateInsId2IOSignalInfo[ctx->componentMemory[mySubcomponents[cmp_index_ref]].templateId].defs[1].offset];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + ((1600 * Fr_toInt(&lvar[3])) + 3200)],1600);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600)){
(*_functionTable[ctx->componentMemory[mySubcomponents[cmp_index_ref]].templateId])(mySubcomponents[cmp_index_ref],ctx);

}
}
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1600 * (Fr_toInt(&lvar[3]) + 1)) + 3200)];
// load src
cmp_index_ref_load = ((1 * Fr_toInt(&lvar[3])) + 0);
cmp_index_ref_load = ((1 * Fr_toInt(&lvar[3])) + 0);
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[((1 * Fr_toInt(&lvar[3])) + 0)]].signalStart + ctx->templateInsId2IOSignalInfo[ctx->componentMemory[mySubcomponents[((1 * Fr_toInt(&lvar[3])) + 0)]].templateId].defs[0].offset],1600);
}
{
PFrElement aux_dest = &lvar[3];
// load src
Fr_add(&expaux[0],&lvar[3],&circuitConstants[2]); // line circom 347
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[11]); // line circom 347
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 41600],1600);
}
for (uint i = 0; i < 24; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakF_59_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 59;
ctx->componentMemory[coffset].templateName = "KeccakF";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[1]{0};
}

void KeccakF_59_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
std::string new_cmp_name = "keccak";
KeccakP_58_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 0+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[11]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
KeccakP_58_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
for (uint i = 0; i < 1; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void LinearizedKeccakF_60_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 60;
ctx->componentMemory[coffset].templateName = "LinearizedKeccakF";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[3]{0};
}

void LinearizedKeccakF_60_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[3];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
std::string new_cmp_name = "dec";
StructureState_2_create(mySignalStart+3200,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "enc";
LinearizeState_3_create(mySignalStart+6400,1+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 1+ctx_index+1;
}
{
std::string new_cmp_name = "kec";
KeccakF_59_create(mySignalStart+9600,2+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 2+ctx_index+1;
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[6]);
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
StructureState_2_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
KeccakF_59_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],1600);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
LinearizeState_3_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],1600);
}
for (uint i = 0; i < 3; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void KeccakSponge_61_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 61;
ctx->componentMemory[coffset].templateName = "KeccakSponge";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 4096;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[4]{0};
}

void KeccakSponge_61_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[5];
FrElement lvar[14];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[3]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[81]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[82]);
}
{
uint aux_create = 0;
int aux_cmp_num = 0+ctx_index+1;
uint csoffset = mySignalStart+21056;
uint aux_dimensions[1] = {4};
for (uint i = 0; i < 4; i++) {
std::string new_cmp_name = "absorb"+ctx->generate_position_array(aux_dimensions, 1, i);
LinearizedKeccakF_60_create(csoffset,aux_cmp_num,ctx,new_cmp_name,myId);
mySubcomponents[aux_create+ i] = aux_cmp_num;
csoffset += 647360 ;
aux_cmp_num += 15509;
}
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[5]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[6]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[83]);
}
{
if (!Fr_isTrue(&circuitConstants[2])) std::cout << "Failed assert in template/function " << myTemplateName << " line 18. " <<  "Followed trace of components: " << ctx->getTrace(myId) << std::endl;
assert(Fr_isTrue(&circuitConstants[2]));
}
{
if (!Fr_isTrue(&circuitConstants[2])) std::cout << "Failed assert in template/function " << myTemplateName << " line 19. " <<  "Followed trace of components: " << ctx->getTrace(myId) << std::endl;
assert(Fr_isTrue(&circuitConstants[2]));
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[9]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[2]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[84]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[10],&circuitConstants[81]); // line circom 30
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[10])) + 4352)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[10])) + 256)]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
Fr_add(&expaux[0],&lvar[10],&circuitConstants[2]); // line circom 30
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[10],&circuitConstants[81]); // line circom 30
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 8448];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[2]);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 8703];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[2]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[86]);
}
Fr_lt(&expaux[0],&lvar[10],&circuitConstants[85]); // line circom 33
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[10])) + 4352)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
Fr_add(&expaux[0],&lvar[10],&circuitConstants[2]); // line circom 33
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[10],&circuitConstants[85]); // line circom 33
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[10],&circuitConstants[6]); // line circom 39
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((0 + (1 * Fr_toInt(&lvar[10]))) + 8704)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
Fr_add(&expaux[0],&lvar[10],&circuitConstants[2]); // line circom 39
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[10],&circuitConstants[6]); // line circom 39
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[10],&circuitConstants[9]); // line circom 44
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[11],&circuitConstants[83]); // line circom 46
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + (((1600 * Fr_toInt(&lvar[10])) + (1 * Fr_toInt(&lvar[11]))) + 8704)]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
Fr_mul(&expaux[1],&lvar[10],&circuitConstants[83]); // line circom 48
Fr_add(&expaux[0],&expaux[1],&lvar[11]); // line circom 48
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((1 * Fr_toInt(&expaux[0])) + 4352)]);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + (((1088 * Fr_toInt(&lvar[10])) + (1 * Fr_toInt(&lvar[11]))) + 16704)];
// load src
Fr_add(&expaux[1],&lvar[12],&lvar[13]); // line circom 49
Fr_mul(&expaux[3],&circuitConstants[10],&lvar[12]); // line circom 49
Fr_mul(&expaux[2],&expaux[3],&lvar[13]); // line circom 49
Fr_sub(&expaux[0],&expaux[1],&expaux[2]); // line circom 49
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
Fr_add(&expaux[0],&lvar[11],&circuitConstants[2]); // line circom 46
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[11],&circuitConstants[83]); // line circom 46
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[11],&circuitConstants[83]); // line circom 53
while(Fr_isTrue(&expaux[0])){
{
uint cmp_index_ref = ((1 * Fr_toInt(&lvar[10])) + 0);
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + ((1 * Fr_toInt(&lvar[11])) + 1600)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + (((1088 * Fr_toInt(&lvar[10])) + (1 * Fr_toInt(&lvar[11]))) + 16704)]);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1)){
LinearizedKeccakF_60_run(mySubcomponents[cmp_index_ref],ctx);

}
}
{
PFrElement aux_dest = &lvar[11];
// load src
Fr_add(&expaux[0],&lvar[11],&circuitConstants[2]); // line circom 53
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[11],&circuitConstants[83]); // line circom 53
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[83]);
}
Fr_lt(&expaux[0],&lvar[11],&circuitConstants[6]); // line circom 54
while(Fr_isTrue(&expaux[0])){
{
uint cmp_index_ref = ((1 * Fr_toInt(&lvar[10])) + 0);
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + ((1 * Fr_toInt(&lvar[11])) + 1600)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + (((1600 * Fr_toInt(&lvar[10])) + (1 * Fr_toInt(&lvar[11]))) + 8704)]);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1)){
LinearizedKeccakF_60_run(mySubcomponents[cmp_index_ref],ctx);

}
}
{
PFrElement aux_dest = &lvar[11];
// load src
Fr_add(&expaux[0],&lvar[11],&circuitConstants[2]); // line circom 54
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[11],&circuitConstants[6]); // line circom 54
}
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1600 * (Fr_toInt(&lvar[10]) + 1)) + 8704)];
// load src
cmp_index_ref_load = ((1 * Fr_toInt(&lvar[10])) + 0);
cmp_index_ref_load = ((1 * Fr_toInt(&lvar[10])) + 0);
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[((1 * Fr_toInt(&lvar[10])) + 0)]].signalStart + 0],1600);
}
{
PFrElement aux_dest = &lvar[10];
// load src
Fr_add(&expaux[0],&lvar[10],&circuitConstants[2]); // line circom 44
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[10],&circuitConstants[9]); // line circom 44
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[82]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[11],&circuitConstants[82]); // line circom 60
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[11])) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((6400 + (1 * Fr_toInt(&lvar[11]))) + 8704)]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
Fr_add(&expaux[0],&lvar[11],&circuitConstants[2]); // line circom 60
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[11],&circuitConstants[82]); // line circom 60
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[83]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[2]);
}
for (uint i = 0; i < 4; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Keccak_256_62_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 62;
ctx->componentMemory[coffset].templateName = "Keccak_256";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 4096;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[1]{0};
}

void Keccak_256_62_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[1];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[81]);
}
{
std::string new_cmp_name = "sponge";
KeccakSponge_61_create(mySignalStart+4352,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 0+ctx_index+1;
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 256];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 256],4096);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 4096;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
KeccakSponge_61_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],256);
}
for (uint i = 0; i < 1; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void PackBytes_63_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 63;
ctx->componentMemory[coffset].templateName = "PackBytes";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 256;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void PackBytes_63_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[4];
FrElement lvar[4];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[35]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[1],&circuitConstants[35]); // line circom 28
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[0]); // line circom 30
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_shl(&expaux[2],&circuitConstants[2],&lvar[3]); // line circom 31
Fr_mul(&expaux[1],&signalValues[mySignalStart + ((1 * ((8 * Fr_toInt(&lvar[1])) + Fr_toInt(&lvar[3]))) + 32)],&expaux[2]); // line circom 31
Fr_add(&expaux[0],&lvar[2],&expaux[1]); // line circom 31
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
Fr_add(&expaux[0],&lvar[3],&circuitConstants[2]); // line circom 30
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[0]); // line circom 30
}
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&lvar[2]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
Fr_add(&expaux[0],&lvar[1],&circuitConstants[2]); // line circom 28
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[1],&circuitConstants[35]); // line circom 28
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Keccak_256_bytes_64_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 64;
ctx->componentMemory[coffset].templateName = "Keccak_256_bytes";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 512;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[3]{0};
}

void Keccak_256_bytes_64_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[1];
FrElement lvar[1];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[3]);
}
{
std::string new_cmp_name = "unpack";
UnpackBytes_1_create(mySignalStart+2615680,62039+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 62039+ctx_index+1;
}
{
std::string new_cmp_name = "keccak";
Keccak_256_62_create(mySignalStart+544,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[1] = 0+ctx_index+1;
}
{
std::string new_cmp_name = "pack";
PackBytes_63_create(mySignalStart+2615392,62038+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[2] = 62038+ctx_index+1;
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 4096];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 32],512);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 512;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
UnpackBytes_1_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 1;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 256];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],4096);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 4096;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Keccak_256_62_run(mySubcomponents[cmp_index_ref],ctx);
}
{
uint cmp_index_ref = 2;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 32];
// load src
cmp_index_ref_load = 1;
cmp_index_ref_load = 1;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[1]].signalStart + 0],256);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 256;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
PackBytes_63_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 2;
cmp_index_ref_load = 2;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[2]].signalStart + 0],32);
}
for (uint i = 0; i < 3; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void Keccak256Hash_65_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 65;
ctx->componentMemory[coffset].templateName = "Keccak256Hash";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 544;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[1]{0};
}

void Keccak256Hash_65_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[2];
FrElement lvar[2];
u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
std::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
std::string myComponentName = ctx->componentMemory[ctx_index].componentName;
u64 myFather = ctx->componentMemory[ctx_index].idFather;
u64 myId = ctx_index;
u32* mySubcomponents = ctx->componentMemory[ctx_index].subcomponents;
bool* mySubcomponentsParallel = ctx->componentMemory[ctx_index].subcomponentsParallel;
std::string* listOfTemplateMessages = ctx->listOfTemplateMessages;
uint sub_component_aux;
uint index_multiple_eq;
int cmp_index_ref_load = -1;
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[3]);
}
{
std::string new_cmp_name = "keccak";
Keccak_256_bytes_64_create(mySignalStart+576,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 0+ctx_index+1;
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 32];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 32],512);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 512;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Keccak_256_bytes_64_run(mySubcomponents[cmp_index_ref],ctx);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
cmp_index_ref_load = 0;
cmp_index_ref_load = 0;
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[0]].signalStart + 0],32);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[1],&circuitConstants[35]); // line circom 25
while(Fr_isTrue(&expaux[0])){
{
{{
Fr_eq(&expaux[0],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 0)],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 544)]); // line circom 26
}}
if (!Fr_isTrue(&expaux[0])) std::cout << "Failed assert in template/function " << myTemplateName << " line 26. " <<  "Followed trace of components: " << ctx->getTrace(myId) << std::endl;
assert(Fr_isTrue(&expaux[0]));
}
{
PFrElement aux_dest = &lvar[1];
// load src
Fr_add(&expaux[0],&lvar[1],&circuitConstants[2]); // line circom 25
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[1],&circuitConstants[35]); // line circom 25
}
for (uint i = 0; i < 1; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0)release_memory_component(ctx,index_subc);
}
}

void run(Circom_CalcWit* ctx){
Keccak256Hash_65_create(1,0,ctx,"main",0);
Keccak256Hash_65_run(0,ctx);
}


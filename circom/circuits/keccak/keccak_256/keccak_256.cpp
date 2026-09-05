#include <stdio.h>
#include <iostream>
#include <assert.h>
#include "circom.hpp"
#include "calcwit.hpp"
void Num2Bits_0_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Num2Bits_0_run(uint ctx_index,Circom_CalcWit* ctx);
void Xor3Bits_1_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Xor3Bits_1_run(uint ctx_index,Circom_CalcWit* ctx);
void Xor3Bits_2_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Xor3Bits_2_run(uint ctx_index,Circom_CalcWit* ctx);
void ChiBits_3_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void ChiBits_3_run(uint ctx_index,Circom_CalcWit* ctx);
void KeccakF1600_4_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void KeccakF1600_4_run(uint ctx_index,Circom_CalcWit* ctx);
void Keccak_256_bytes_5_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Keccak_256_bytes_5_run(uint ctx_index,Circom_CalcWit* ctx);
void Keccak256Hash_6_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather);
void Keccak256Hash_6_run(uint ctx_index,Circom_CalcWit* ctx);
Circom_TemplateFunction _functionTable[7] = { 
Num2Bits_0_run,
Xor3Bits_1_run,
Xor3Bits_2_run,
ChiBits_3_run,
KeccakF1600_4_run,
Keccak_256_bytes_5_run,
Keccak256Hash_6_run };
Circom_TemplateFunction _functionTableParallel[7] = { 
NULL,
NULL,
NULL,
NULL,
NULL,
NULL,
NULL };
uint get_main_input_signal_start() {return 33;}

uint get_main_input_signal_no() {return 288;}

uint get_total_signal_no() {return 905569;}

uint get_number_of_components() {return 836;}

uint get_size_of_input_hashmap() {return 256;}

uint get_size_of_witness() {return 187329;}

uint get_size_of_constants() {return 68;}

uint get_size_of_io_map() {return 0;}

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
// template declarations
void Num2Bits_0_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 0;
ctx->componentMemory[coffset].templateName = "Num2Bits";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Num2Bits_0_run(uint ctx_index,Circom_CalcWit* ctx){
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
Fr_copy(aux_dest,&circuitConstants[2]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[0]); // line circom 31
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[3])) + 0)];
// load src
Fr_shr(&expaux[1],&signalValues[mySignalStart + 8],&lvar[3]); // line circom 32
Fr_band(&expaux[0],&expaux[1],&circuitConstants[2]); // line circom 32
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
Fr_sub(&expaux[2],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[3])) + 0)],&circuitConstants[2]); // line circom 33
Fr_mul(&expaux[1],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[3])) + 0)],&expaux[2]); // line circom 33
{{
Fr_eq(&expaux[0],&expaux[1],&circuitConstants[1]); // line circom 33
}}
if (!Fr_isTrue(&expaux[0])) std::cout << "Failed assert in template/function " << myTemplateName << " line 33. " <<  "Followed trace of components: " << ctx->getTrace(myId) << std::endl;
assert(Fr_isTrue(&expaux[0]));
}
{
PFrElement aux_dest = &lvar[1];
// load src
Fr_mul(&expaux[1],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[3])) + 0)],&lvar[2]); // line circom 34
Fr_add(&expaux[0],&lvar[1],&expaux[1]); // line circom 34
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_add(&expaux[0],&lvar[2],&lvar[2]); // line circom 35
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
Fr_add(&expaux[0],&lvar[3],&circuitConstants[2]); // line circom 31
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[0]); // line circom 31
}
{
{{
Fr_eq(&expaux[0],&lvar[1],&signalValues[mySignalStart + 8]); // line circom 38
}}
if (!Fr_isTrue(&expaux[0])) std::cout << "Failed assert in template/function " << myTemplateName << " line 38. " <<  "Followed trace of components: " << ctx->getTrace(myId) << std::endl;
assert(Fr_isTrue(&expaux[0]));
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0){
assert(!(ctx->componentMemory[index_subc].inputCounter));
release_memory_component(ctx,index_subc);
}
}
}

void Xor3Bits_1_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 1;
ctx->componentMemory[coffset].templateName = "Xor3Bits";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 192;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Xor3Bits_1_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[8];
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
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[1],&circuitConstants[3]); // line circom 14
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 0)];
// load src
Fr_bxor(&expaux[1],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 64)],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 128)]); // line circom 15
Fr_bxor(&expaux[0],&expaux[1],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 192)]); // line circom 15
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
Fr_mul(&expaux[5],&circuitConstants[4],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 64)]); // line circom 17
Fr_add(&expaux[4],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 0)],&expaux[5]); // line circom 17
Fr_mul(&expaux[5],&circuitConstants[4],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 128)]); // line circom 17
Fr_add(&expaux[3],&expaux[4],&expaux[5]); // line circom 17
Fr_mul(&expaux[4],&circuitConstants[5],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 192)]); // line circom 17
Fr_add(&expaux[2],&expaux[3],&expaux[4]); // line circom 17
Fr_add(&expaux[5],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 64)],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 128)]); // line circom 18
Fr_mul(&expaux[6],&circuitConstants[6],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 192)]); // line circom 18
Fr_sub(&expaux[4],&expaux[5],&expaux[6]); // line circom 18
Fr_add(&expaux[3],&expaux[4],&circuitConstants[2]); // line circom 18
Fr_mul(&expaux[1],&expaux[2],&expaux[3]); // line circom 17
Fr_mul(&expaux[4],&circuitConstants[7],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 64)]); // line circom 18
Fr_mul(&expaux[5],&circuitConstants[7],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 128)]); // line circom 18
Fr_add(&expaux[3],&expaux[4],&expaux[5]); // line circom 18
Fr_mul(&expaux[4],&circuitConstants[8],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 192)]); // line circom 18
Fr_sub(&expaux[2],&expaux[3],&expaux[4]); // line circom 18
{{
Fr_eq(&expaux[0],&expaux[1],&expaux[2]); // line circom 17
}}
if (!Fr_isTrue(&expaux[0])) std::cout << "Failed assert in template/function " << myTemplateName << " line 17. " <<  "Followed trace of components: " << ctx->getTrace(myId) << std::endl;
assert(Fr_isTrue(&expaux[0]));
}
{
PFrElement aux_dest = &lvar[1];
// load src
Fr_add(&expaux[0],&lvar[1],&circuitConstants[2]); // line circom 14
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[1],&circuitConstants[3]); // line circom 14
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0){
assert(!(ctx->componentMemory[index_subc].inputCounter));
release_memory_component(ctx,index_subc);
}
}
}

void Xor3Bits_2_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 2;
ctx->componentMemory[coffset].templateName = "Xor3Bits";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 4800;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void Xor3Bits_2_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[8];
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
Fr_copy(aux_dest,&circuitConstants[9]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[1],&circuitConstants[9]); // line circom 14
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 0)];
// load src
Fr_bxor(&expaux[1],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 1600)],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 3200)]); // line circom 15
Fr_bxor(&expaux[0],&expaux[1],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 4800)]); // line circom 15
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
Fr_mul(&expaux[5],&circuitConstants[4],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 1600)]); // line circom 17
Fr_add(&expaux[4],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 0)],&expaux[5]); // line circom 17
Fr_mul(&expaux[5],&circuitConstants[4],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 3200)]); // line circom 17
Fr_add(&expaux[3],&expaux[4],&expaux[5]); // line circom 17
Fr_mul(&expaux[4],&circuitConstants[5],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 4800)]); // line circom 17
Fr_add(&expaux[2],&expaux[3],&expaux[4]); // line circom 17
Fr_add(&expaux[5],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 1600)],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 3200)]); // line circom 18
Fr_mul(&expaux[6],&circuitConstants[6],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 4800)]); // line circom 18
Fr_sub(&expaux[4],&expaux[5],&expaux[6]); // line circom 18
Fr_add(&expaux[3],&expaux[4],&circuitConstants[2]); // line circom 18
Fr_mul(&expaux[1],&expaux[2],&expaux[3]); // line circom 17
Fr_mul(&expaux[4],&circuitConstants[7],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 1600)]); // line circom 18
Fr_mul(&expaux[5],&circuitConstants[7],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 3200)]); // line circom 18
Fr_add(&expaux[3],&expaux[4],&expaux[5]); // line circom 18
Fr_mul(&expaux[4],&circuitConstants[8],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 4800)]); // line circom 18
Fr_sub(&expaux[2],&expaux[3],&expaux[4]); // line circom 18
{{
Fr_eq(&expaux[0],&expaux[1],&expaux[2]); // line circom 17
}}
if (!Fr_isTrue(&expaux[0])) std::cout << "Failed assert in template/function " << myTemplateName << " line 17. " <<  "Followed trace of components: " << ctx->getTrace(myId) << std::endl;
assert(Fr_isTrue(&expaux[0]));
}
{
PFrElement aux_dest = &lvar[1];
// load src
Fr_add(&expaux[0],&lvar[1],&circuitConstants[2]); // line circom 14
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[1],&circuitConstants[9]); // line circom 14
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0){
assert(!(ctx->componentMemory[index_subc].inputCounter));
release_memory_component(ctx,index_subc);
}
}
}

void ChiBits_3_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 3;
ctx->componentMemory[coffset].templateName = "ChiBits";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 4800;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[0];
}

void ChiBits_3_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[8];
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
Fr_copy(aux_dest,&circuitConstants[9]);
}
{
PFrElement aux_dest = &lvar[1];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[1],&circuitConstants[9]); // line circom 40
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 0)];
// load src
Fr_sub(&expaux[2],&circuitConstants[2],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 3200)]); // line circom 41
Fr_band(&expaux[1],&expaux[2],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 4800)]); // line circom 41
Fr_bxor(&expaux[0],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 1600)],&expaux[1]); // line circom 41
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
Fr_mul(&expaux[5],&circuitConstants[10],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 1600)]); // line circom 43
Fr_add(&expaux[4],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 0)],&expaux[5]); // line circom 43
Fr_sub(&expaux[3],&expaux[4],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 3200)]); // line circom 43
Fr_sub(&expaux[2],&expaux[3],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 4800)]); // line circom 43
Fr_mul(&expaux[6],&circuitConstants[6],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 1600)]); // line circom 44
Fr_add(&expaux[5],&expaux[6],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 3200)]); // line circom 44
Fr_add(&expaux[4],&expaux[5],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 4800)]); // line circom 44
Fr_sub(&expaux[3],&expaux[4],&circuitConstants[10]); // line circom 44
Fr_mul(&expaux[1],&expaux[2],&expaux[3]); // line circom 43
Fr_mul(&expaux[3],&circuitConstants[6],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 1600)]); // line circom 44
Fr_mul(&expaux[4],&circuitConstants[4],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 3200)]); // line circom 44
Fr_add(&expaux[2],&expaux[3],&expaux[4]); // line circom 44
{{
Fr_eq(&expaux[0],&expaux[1],&expaux[2]); // line circom 43
}}
if (!Fr_isTrue(&expaux[0])) std::cout << "Failed assert in template/function " << myTemplateName << " line 43. " <<  "Followed trace of components: " << ctx->getTrace(myId) << std::endl;
assert(Fr_isTrue(&expaux[0]));
}
{
PFrElement aux_dest = &lvar[1];
// load src
Fr_add(&expaux[0],&lvar[1],&circuitConstants[2]); // line circom 40
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[1],&circuitConstants[9]); // line circom 40
}
for (uint i = 0; i < 0; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0){
assert(!(ctx->componentMemory[index_subc].inputCounter));
release_memory_component(ctx,index_subc);
}
}
}

void KeccakF1600_4_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 4;
ctx->componentMemory[coffset].templateName = "KeccakF1600";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 1600;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[288]{0};
}

void KeccakF1600_4_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[11];
FrElement lvar[53];
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
uint aux_create = 0;
int aux_cmp_num = 24+ctx_index+1;
uint csoffset = mySignalStart+235200;
uint aux_dimensions[2] = {24,5};
for (uint i = 0; i < 120; i++) {
std::string new_cmp_name = "parity3"+ctx->generate_position_array(aux_dimensions, 2, i);
Xor3Bits_1_create(csoffset,aux_cmp_num,ctx,new_cmp_name,myId);
mySubcomponents[aux_create+ i] = aux_cmp_num;
csoffset += 256 ;
aux_cmp_num += 1;
}
}
{
uint aux_create = 120;
int aux_cmp_num = 144+ctx_index+1;
uint csoffset = mySignalStart+265920;
uint aux_dimensions[2] = {24,5};
for (uint i = 0; i < 120; i++) {
std::string new_cmp_name = "parity5"+ctx->generate_position_array(aux_dimensions, 2, i);
Xor3Bits_1_create(csoffset,aux_cmp_num,ctx,new_cmp_name,myId);
mySubcomponents[aux_create+ i] = aux_cmp_num;
csoffset += 256 ;
aux_cmp_num += 1;
}
}
{
uint aux_create = 240;
int aux_cmp_num = 264+ctx_index+1;
uint csoffset = mySignalStart+296640;
uint aux_dimensions[1] = {24};
for (uint i = 0; i < 24; i++) {
std::string new_cmp_name = "theta"+ctx->generate_position_array(aux_dimensions, 1, i);
Xor3Bits_2_create(csoffset,aux_cmp_num,ctx,new_cmp_name,myId);
mySubcomponents[aux_create+ i] = aux_cmp_num;
csoffset += 6400 ;
aux_cmp_num += 1;
}
}
{
uint aux_create = 264;
int aux_cmp_num = 0+ctx_index+1;
uint csoffset = mySignalStart+81600;
uint aux_dimensions[1] = {24};
for (uint i = 0; i < 24; i++) {
std::string new_cmp_name = "chi"+ctx->generate_position_array(aux_dimensions, 1, i);
ChiBits_3_create(csoffset,aux_cmp_num,ctx,new_cmp_name,myId);
mySubcomponents[aux_create+ i] = aux_cmp_num;
csoffset += 6400 ;
aux_cmp_num += 1;
}
}
{
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
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
PFrElement aux_dest = &lvar[0];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
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
Fr_copy(aux_dest,&circuitConstants[27]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[28]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[29]);
}
{
PFrElement aux_dest = &lvar[5];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[30]);
}
{
PFrElement aux_dest = &lvar[6];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[31]);
}
{
PFrElement aux_dest = &lvar[7];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[7]);
}
{
PFrElement aux_dest = &lvar[8];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[32]);
}
{
PFrElement aux_dest = &lvar[9];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[23]);
}
{
PFrElement aux_dest = &lvar[10];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[10]);
}
{
PFrElement aux_dest = &lvar[11];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[13]);
}
{
PFrElement aux_dest = &lvar[12];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[33]);
}
{
PFrElement aux_dest = &lvar[13];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[34]);
}
{
PFrElement aux_dest = &lvar[14];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[35]);
}
{
PFrElement aux_dest = &lvar[15];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[36]);
}
{
PFrElement aux_dest = &lvar[16];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[37]);
}
{
PFrElement aux_dest = &lvar[17];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[18]);
}
{
PFrElement aux_dest = &lvar[18];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[24]);
}
{
PFrElement aux_dest = &lvar[19];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[0]);
}
{
PFrElement aux_dest = &lvar[20];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[21]);
}
{
PFrElement aux_dest = &lvar[21];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[4]);
}
{
PFrElement aux_dest = &lvar[22];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[38]);
}
{
PFrElement aux_dest = &lvar[23];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[39]);
}
{
PFrElement aux_dest = &lvar[24];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[17]);
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
PFrElement aux_dest = &lvar[25];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[2]);
}
{
PFrElement aux_dest = &lvar[26];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[40]);
}
{
PFrElement aux_dest = &lvar[27];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[41]);
}
{
PFrElement aux_dest = &lvar[28];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[42]);
}
{
PFrElement aux_dest = &lvar[29];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[43]);
}
{
PFrElement aux_dest = &lvar[30];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[44]);
}
{
PFrElement aux_dest = &lvar[31];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[45]);
}
{
PFrElement aux_dest = &lvar[32];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[46]);
}
{
PFrElement aux_dest = &lvar[33];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[47]);
}
{
PFrElement aux_dest = &lvar[34];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[48]);
}
{
PFrElement aux_dest = &lvar[35];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[49]);
}
{
PFrElement aux_dest = &lvar[36];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[50]);
}
{
PFrElement aux_dest = &lvar[37];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[51]);
}
{
PFrElement aux_dest = &lvar[38];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[52]);
}
{
PFrElement aux_dest = &lvar[39];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[53]);
}
{
PFrElement aux_dest = &lvar[40];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[54]);
}
{
PFrElement aux_dest = &lvar[41];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[55]);
}
{
PFrElement aux_dest = &lvar[42];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[56]);
}
{
PFrElement aux_dest = &lvar[43];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[57]);
}
{
PFrElement aux_dest = &lvar[44];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[58]);
}
{
PFrElement aux_dest = &lvar[45];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[45]);
}
{
PFrElement aux_dest = &lvar[46];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[59]);
}
{
PFrElement aux_dest = &lvar[47];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[44]);
}
{
PFrElement aux_dest = &lvar[48];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[60]);
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 3200];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 1600],1600);
}
{
PFrElement aux_dest = &lvar[49];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[49],&circuitConstants[8]); // line circom 35
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[50],&circuitConstants[11]); // line circom 36
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[51],&circuitConstants[3]); // line circom 39
while(Fr_isTrue(&expaux[0])){
{
uint cmp_index_ref = (((5 * Fr_toInt(&lvar[49])) + (1 * Fr_toInt(&lvar[50]))) + 0);
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + ((1 * Fr_toInt(&lvar[51])) + 64)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + (((1600 * Fr_toInt(&lvar[49])) + (1 * ((64 * Fr_toInt(&lvar[50])) + Fr_toInt(&lvar[51])))) + 3200)]);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1)){
Xor3Bits_1_run(mySubcomponents[cmp_index_ref],ctx);

}
}
{
uint cmp_index_ref = (((5 * Fr_toInt(&lvar[49])) + (1 * Fr_toInt(&lvar[50]))) + 0);
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + ((1 * Fr_toInt(&lvar[51])) + 128)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + (((1600 * Fr_toInt(&lvar[49])) + (1 * ((64 * (Fr_toInt(&lvar[50]) + 5)) + Fr_toInt(&lvar[51])))) + 3200)]);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1)){
Xor3Bits_1_run(mySubcomponents[cmp_index_ref],ctx);

}
}
{
uint cmp_index_ref = (((5 * Fr_toInt(&lvar[49])) + (1 * Fr_toInt(&lvar[50]))) + 0);
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + ((1 * Fr_toInt(&lvar[51])) + 192)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + (((1600 * Fr_toInt(&lvar[49])) + (1 * ((64 * (Fr_toInt(&lvar[50]) + 10)) + Fr_toInt(&lvar[51])))) + 3200)]);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1)){
Xor3Bits_1_run(mySubcomponents[cmp_index_ref],ctx);

}
}
{
PFrElement aux_dest = &lvar[51];
// load src
Fr_add(&expaux[0],&lvar[51],&circuitConstants[2]); // line circom 39
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[51],&circuitConstants[3]); // line circom 39
}
{
uint cmp_index_ref = (((5 * Fr_toInt(&lvar[49])) + (1 * Fr_toInt(&lvar[50]))) + 120);
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 64];
// load src
cmp_index_ref_load = (((5 * Fr_toInt(&lvar[49])) + (1 * Fr_toInt(&lvar[50]))) + 0);
cmp_index_ref_load = (((5 * Fr_toInt(&lvar[49])) + (1 * Fr_toInt(&lvar[50]))) + 0);
// end load src
Fr_copyn(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[(((5 * Fr_toInt(&lvar[49])) + (1 * Fr_toInt(&lvar[50]))) + 0)]].signalStart + 0],64);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 64)){
Xor3Bits_1_run(mySubcomponents[cmp_index_ref],ctx);

}
}
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[51],&circuitConstants[3]); // line circom 45
while(Fr_isTrue(&expaux[0])){
{
uint cmp_index_ref = (((5 * Fr_toInt(&lvar[49])) + (1 * Fr_toInt(&lvar[50]))) + 120);
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + ((1 * Fr_toInt(&lvar[51])) + 128)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + (((1600 * Fr_toInt(&lvar[49])) + (1 * ((64 * (Fr_toInt(&lvar[50]) + 15)) + Fr_toInt(&lvar[51])))) + 3200)]);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1)){
Xor3Bits_1_run(mySubcomponents[cmp_index_ref],ctx);

}
}
{
uint cmp_index_ref = (((5 * Fr_toInt(&lvar[49])) + (1 * Fr_toInt(&lvar[50]))) + 120);
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + ((1 * Fr_toInt(&lvar[51])) + 192)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + (((1600 * Fr_toInt(&lvar[49])) + (1 * ((64 * (Fr_toInt(&lvar[50]) + 20)) + Fr_toInt(&lvar[51])))) + 3200)]);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1)){
Xor3Bits_1_run(mySubcomponents[cmp_index_ref],ctx);

}
}
{
PFrElement aux_dest = &lvar[51];
// load src
Fr_add(&expaux[0],&lvar[51],&circuitConstants[2]); // line circom 45
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[51],&circuitConstants[3]); // line circom 45
}
{
PFrElement aux_dest = &lvar[50];
// load src
Fr_add(&expaux[0],&lvar[50],&circuitConstants[2]); // line circom 36
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[50],&circuitConstants[11]); // line circom 36
}
{
uint cmp_index_ref = ((1 * Fr_toInt(&lvar[49])) + 240);
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + ((1600 * Fr_toInt(&lvar[49])) + 3200)],1600);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600)){
Xor3Bits_2_run(mySubcomponents[cmp_index_ref],ctx);

}
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[50],&circuitConstants[11]); // line circom 52
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[51],&circuitConstants[11]); // line circom 53
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[52],&circuitConstants[3]); // line circom 54
while(Fr_isTrue(&expaux[0])){
{
uint cmp_index_ref = ((1 * Fr_toInt(&lvar[49])) + 240);
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + ((1 * ((64 * (Fr_toInt(&lvar[50]) + (5 * Fr_toInt(&lvar[51])))) + Fr_toInt(&lvar[52]))) + 3200)];
// load src
Fr_add(&expaux[1],&lvar[50],&circuitConstants[6]); // line circom 55
Fr_mod(&expaux[0],&expaux[1],&circuitConstants[11]); // line circom 55
cmp_index_ref_load = (((5 * Fr_toInt(&lvar[49])) + (1 * Fr_toInt(&expaux[0]))) + 120);
cmp_index_ref_load = (((5 * Fr_toInt(&lvar[49])) + (1 * Fr_toInt(&expaux[0]))) + 120);
// end load src
Fr_copy(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[(((5 * Fr_toInt(&lvar[49])) + (1 * Fr_toInt(&expaux[0]))) + 120)]].signalStart + ((1 * Fr_toInt(&lvar[52])) + 0)]);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1)){
Xor3Bits_2_run(mySubcomponents[cmp_index_ref],ctx);

}
}
{
uint cmp_index_ref = ((1 * Fr_toInt(&lvar[49])) + 240);
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + ((1 * ((64 * (Fr_toInt(&lvar[50]) + (5 * Fr_toInt(&lvar[51])))) + Fr_toInt(&lvar[52]))) + 4800)];
// load src
Fr_add(&expaux[1],&lvar[50],&circuitConstants[2]); // line circom 56
Fr_mod(&expaux[0],&expaux[1],&circuitConstants[11]); // line circom 56
Fr_add(&expaux[2],&lvar[52],&circuitConstants[61]); // line circom 56
Fr_mod(&expaux[1],&expaux[2],&circuitConstants[3]); // line circom 56
cmp_index_ref_load = (((5 * Fr_toInt(&lvar[49])) + (1 * Fr_toInt(&expaux[0]))) + 120);
cmp_index_ref_load = (((5 * Fr_toInt(&lvar[49])) + (1 * Fr_toInt(&expaux[0]))) + 120);
// end load src
Fr_copy(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[(((5 * Fr_toInt(&lvar[49])) + (1 * Fr_toInt(&expaux[0]))) + 120)]].signalStart + ((1 * Fr_toInt(&expaux[1])) + 0)]);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1)){
Xor3Bits_2_run(mySubcomponents[cmp_index_ref],ctx);

}
}
{
PFrElement aux_dest = &lvar[52];
// load src
Fr_add(&expaux[0],&lvar[52],&circuitConstants[2]); // line circom 54
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[52],&circuitConstants[3]); // line circom 54
}
{
PFrElement aux_dest = &lvar[51];
// load src
Fr_add(&expaux[0],&lvar[51],&circuitConstants[2]); // line circom 53
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[51],&circuitConstants[11]); // line circom 53
}
{
PFrElement aux_dest = &lvar[50];
// load src
Fr_add(&expaux[0],&lvar[50],&circuitConstants[2]); // line circom 52
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[50],&circuitConstants[11]); // line circom 52
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[50],&circuitConstants[11]); // line circom 61
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[51],&circuitConstants[11]); // line circom 62
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[52],&circuitConstants[3]); // line circom 63
while(Fr_isTrue(&expaux[0])){
Fr_mul(&expaux[8],&circuitConstants[4],&lvar[50]); // line circom 64
Fr_mul(&expaux[9],&circuitConstants[10],&lvar[51]); // line circom 64
Fr_add(&expaux[7],&expaux[8],&expaux[9]); // line circom 64
Fr_mod(&expaux[6],&expaux[7],&circuitConstants[11]); // line circom 64
Fr_mul(&expaux[4],&circuitConstants[11],&expaux[6]); // line circom 64
Fr_add(&expaux[3],&lvar[51],&expaux[4]); // line circom 64
Fr_mul(&expaux[1],&circuitConstants[3],&expaux[3]); // line circom 64
Fr_add(&expaux[0],&expaux[1],&lvar[52]); // line circom 64
{
PFrElement aux_dest = &signalValues[mySignalStart + (((1600 * Fr_toInt(&lvar[49])) + (1 * Fr_toInt(&expaux[0]))) + 43200)];
// load src
Fr_mul(&expaux[4],&circuitConstants[11],&lvar[51]); // line circom 65
Fr_add(&expaux[3],&lvar[50],&expaux[4]); // line circom 65
Fr_mul(&expaux[1],&circuitConstants[3],&expaux[3]); // line circom 65
Fr_add(&expaux[4],&lvar[52],&circuitConstants[3]); // line circom 65
Fr_sub(&expaux[3],&expaux[4],&lvar[((1 * (Fr_toInt(&lvar[50]) + (5 * Fr_toInt(&lvar[51])))) + 0)]); // line circom 65
Fr_mod(&expaux[2],&expaux[3],&circuitConstants[3]); // line circom 65
Fr_add(&expaux[0],&expaux[1],&expaux[2]); // line circom 65
cmp_index_ref_load = ((1 * Fr_toInt(&lvar[49])) + 240);
cmp_index_ref_load = ((1 * Fr_toInt(&lvar[49])) + 240);
// end load src
Fr_copy(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[((1 * Fr_toInt(&lvar[49])) + 240)]].signalStart + ((1 * Fr_toInt(&expaux[0])) + 0)]);
}
{
PFrElement aux_dest = &lvar[52];
// load src
Fr_add(&expaux[0],&lvar[52],&circuitConstants[2]); // line circom 63
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[52],&circuitConstants[3]); // line circom 63
}
{
PFrElement aux_dest = &lvar[51];
// load src
Fr_add(&expaux[0],&lvar[51],&circuitConstants[2]); // line circom 62
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[51],&circuitConstants[11]); // line circom 62
}
{
PFrElement aux_dest = &lvar[50];
// load src
Fr_add(&expaux[0],&lvar[50],&circuitConstants[2]); // line circom 61
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[50],&circuitConstants[11]); // line circom 61
}
{
uint cmp_index_ref = ((1 * Fr_toInt(&lvar[49])) + 264);
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 1600];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + ((1600 * Fr_toInt(&lvar[49])) + 43200)],1600);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1600)){
ChiBits_3_run(mySubcomponents[cmp_index_ref],ctx);

}
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[50],&circuitConstants[11]); // line circom 71
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[51];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[51],&circuitConstants[11]); // line circom 72
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[52];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[52],&circuitConstants[3]); // line circom 73
while(Fr_isTrue(&expaux[0])){
{
uint cmp_index_ref = ((1 * Fr_toInt(&lvar[49])) + 264);
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + ((1 * ((64 * (Fr_toInt(&lvar[50]) + (5 * Fr_toInt(&lvar[51])))) + Fr_toInt(&lvar[52]))) + 3200)];
// load src
Fr_add(&expaux[5],&lvar[50],&circuitConstants[2]); // line circom 74
Fr_mod(&expaux[4],&expaux[5],&circuitConstants[11]); // line circom 74
Fr_mul(&expaux[5],&circuitConstants[11],&lvar[51]); // line circom 74
Fr_add(&expaux[3],&expaux[4],&expaux[5]); // line circom 74
Fr_mul(&expaux[1],&circuitConstants[3],&expaux[3]); // line circom 74
Fr_add(&expaux[0],&expaux[1],&lvar[52]); // line circom 74
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + (((1600 * Fr_toInt(&lvar[49])) + (1 * Fr_toInt(&expaux[0]))) + 43200)]);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1)){
ChiBits_3_run(mySubcomponents[cmp_index_ref],ctx);

}
}
{
uint cmp_index_ref = ((1 * Fr_toInt(&lvar[49])) + 264);
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + ((1 * ((64 * (Fr_toInt(&lvar[50]) + (5 * Fr_toInt(&lvar[51])))) + Fr_toInt(&lvar[52]))) + 4800)];
// load src
Fr_add(&expaux[5],&lvar[50],&circuitConstants[4]); // line circom 75
Fr_mod(&expaux[4],&expaux[5],&circuitConstants[11]); // line circom 75
Fr_mul(&expaux[5],&circuitConstants[11],&lvar[51]); // line circom 75
Fr_add(&expaux[3],&expaux[4],&expaux[5]); // line circom 75
Fr_mul(&expaux[1],&circuitConstants[3],&expaux[3]); // line circom 75
Fr_add(&expaux[0],&expaux[1],&lvar[52]); // line circom 75
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + (((1600 * Fr_toInt(&lvar[49])) + (1 * Fr_toInt(&expaux[0]))) + 43200)]);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1)){
ChiBits_3_run(mySubcomponents[cmp_index_ref],ctx);

}
}
{
PFrElement aux_dest = &lvar[52];
// load src
Fr_add(&expaux[0],&lvar[52],&circuitConstants[2]); // line circom 73
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[52],&circuitConstants[3]); // line circom 73
}
{
PFrElement aux_dest = &lvar[51];
// load src
Fr_add(&expaux[0],&lvar[51],&circuitConstants[2]); // line circom 72
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[51],&circuitConstants[11]); // line circom 72
}
{
PFrElement aux_dest = &lvar[50];
// load src
Fr_add(&expaux[0],&lvar[50],&circuitConstants[2]); // line circom 71
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[50],&circuitConstants[11]); // line circom 71
}
{
PFrElement aux_dest = &lvar[50];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[50],&circuitConstants[9]); // line circom 79
while(Fr_isTrue(&expaux[0])){
Fr_lt(&expaux[1],&lvar[50],&circuitConstants[3]); // line circom 80
Fr_shr(&expaux[3],&lvar[((1 * Fr_toInt(&lvar[49])) + 25)],&lvar[50]); // line circom 80
Fr_band(&expaux[2],&expaux[3],&circuitConstants[2]); // line circom 80
Fr_land(&expaux[0],&expaux[1],&expaux[2]); // line circom 80
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + (((1600 * (Fr_toInt(&lvar[49]) + 1)) + (1 * Fr_toInt(&lvar[50]))) + 3200)];
// load src
cmp_index_ref_load = ((1 * Fr_toInt(&lvar[49])) + 264);
cmp_index_ref_load = ((1 * Fr_toInt(&lvar[49])) + 264);
Fr_sub(&expaux[0],&circuitConstants[2],&ctx->signalValues[ctx->componentMemory[mySubcomponents[((1 * Fr_toInt(&lvar[49])) + 264)]].signalStart + ((1 * Fr_toInt(&lvar[50])) + 0)]); // line circom 81
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + (((1600 * (Fr_toInt(&lvar[49]) + 1)) + (1 * Fr_toInt(&lvar[50]))) + 3200)];
// load src
cmp_index_ref_load = ((1 * Fr_toInt(&lvar[49])) + 264);
cmp_index_ref_load = ((1 * Fr_toInt(&lvar[49])) + 264);
// end load src
Fr_copy(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[((1 * Fr_toInt(&lvar[49])) + 264)]].signalStart + ((1 * Fr_toInt(&lvar[50])) + 0)]);
}
}
{
PFrElement aux_dest = &lvar[50];
// load src
Fr_add(&expaux[0],&lvar[50],&circuitConstants[2]); // line circom 79
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[50],&circuitConstants[9]); // line circom 79
}
{
PFrElement aux_dest = &lvar[49];
// load src
Fr_add(&expaux[0],&lvar[49],&circuitConstants[2]); // line circom 35
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[49],&circuitConstants[8]); // line circom 35
}
{
PFrElement aux_dest = &signalValues[mySignalStart + 0];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 41600],1600);
}
for (uint i = 0; i < 288; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0){
assert(!(ctx->componentMemory[index_subc].inputCounter));
release_memory_component(ctx,index_subc);
}
}
}

void Keccak_256_bytes_5_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 5;
ctx->componentMemory[coffset].templateName = "Keccak_256_bytes";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 256;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[258]{0};
}

void Keccak_256_bytes_5_run(uint ctx_index,Circom_CalcWit* ctx){
FrElement* circuitConstants = ctx->circuitConstants;
FrElement* signalValues = ctx->signalValues;
FrElement expaux[5];
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
Fr_copy(aux_dest,&circuitConstants[62]);
}
{
uint aux_create = 0;
int aux_cmp_num = 0+ctx_index+1;
uint csoffset = mySignalStart+2464;
uint aux_dimensions[1] = {256};
for (uint i = 0; i < 256; i++) {
std::string new_cmp_name = "bytes"+ctx->generate_position_array(aux_dimensions, 1, i);
Num2Bits_0_create(csoffset,aux_cmp_num,ctx,new_cmp_name,myId);
mySubcomponents[aux_create+ i] = aux_cmp_num;
csoffset += 9 ;
aux_cmp_num += 1;
}
}
{
uint aux_create = 256;
int aux_cmp_num = 256+ctx_index+1;
uint csoffset = mySignalStart+4768;
uint aux_dimensions[1] = {2};
for (uint i = 0; i < 2; i++) {
std::string new_cmp_name = "permutation"+ctx->generate_position_array(aux_dimensions, 1, i);
KeccakF1600_4_create(csoffset,aux_cmp_num,ctx,new_cmp_name,myId);
mySubcomponents[aux_create+ i] = aux_cmp_num;
csoffset += 450240 ;
aux_cmp_num += 289;
}
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
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[62]); // line circom 13
while(Fr_isTrue(&expaux[0])){
{
uint cmp_index_ref = ((1 * Fr_toInt(&lvar[2])) + 0);
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 8];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[2])) + 32)]);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1)){
Num2Bits_0_run(mySubcomponents[cmp_index_ref],ctx);

}
}
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[0]); // line circom 16
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1 * ((8 * Fr_toInt(&lvar[2])) + Fr_toInt(&lvar[3]))) + 288)];
// load src
cmp_index_ref_load = ((1 * Fr_toInt(&lvar[2])) + 0);
cmp_index_ref_load = ((1 * Fr_toInt(&lvar[2])) + 0);
// end load src
Fr_copy(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[((1 * Fr_toInt(&lvar[2])) + 0)]].signalStart + ((1 * Fr_toInt(&lvar[3])) + 0)]);
}
{
PFrElement aux_dest = &lvar[3];
// load src
Fr_add(&expaux[0],&lvar[3],&circuitConstants[2]); // line circom 16
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[0]); // line circom 16
}
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_add(&expaux[0],&lvar[2],&circuitConstants[2]); // line circom 13
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[62]); // line circom 13
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[63]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[64]); // line circom 18
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[1],&lvar[2],&circuitConstants[63]); // line circom 19
}}
{{
Fr_eq(&expaux[2],&lvar[2],&circuitConstants[65]); // line circom 19
}}
Fr_lor(&expaux[0],&expaux[1],&expaux[2]); // line circom 19
if(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[2])) + 288)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[2]);
}
}else{
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[2])) + 288)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
}
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_add(&expaux[0],&lvar[2],&circuitConstants[2]); // line circom 18
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[64]); // line circom 18
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[4]); // line circom 26
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[3];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[9]); // line circom 28
while(Fr_isTrue(&expaux[0])){
{{
Fr_eq(&expaux[0],&lvar[2],&circuitConstants[1]); // line circom 29
}}
if(Fr_isTrue(&expaux[0])){
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[66]); // line circom 30
if(Fr_isTrue(&expaux[0])){
{
uint cmp_index_ref = 256;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + ((1 * Fr_toInt(&lvar[3])) + 1600)];
// load src
// end load src
Fr_copy(aux_dest,&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[3])) + 288)]);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1)){
KeccakF1600_4_run(mySubcomponents[cmp_index_ref],ctx);

}
}
}else{
{
uint cmp_index_ref = 256;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + ((1 * Fr_toInt(&lvar[3])) + 1600)];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1)){
KeccakF1600_4_run(mySubcomponents[cmp_index_ref],ctx);

}
}
}
}else{
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[66]); // line circom 36
if(Fr_isTrue(&expaux[0])){
{
uint cmp_index_ref = 257;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + ((1 * Fr_toInt(&lvar[3])) + 1600)];
// load src
cmp_index_ref_load = 256;
cmp_index_ref_load = 256;
Fr_add(&expaux[1],&ctx->signalValues[ctx->componentMemory[mySubcomponents[256]].signalStart + ((1 * Fr_toInt(&lvar[3])) + 0)],&signalValues[mySignalStart + ((1 * (1088 + Fr_toInt(&lvar[3]))) + 288)]); // line circom 37
cmp_index_ref_load = 256;
cmp_index_ref_load = 256;
Fr_mul(&expaux[3],&circuitConstants[4],&ctx->signalValues[ctx->componentMemory[mySubcomponents[256]].signalStart + ((1 * Fr_toInt(&lvar[3])) + 0)]); // line circom 38
Fr_mul(&expaux[2],&expaux[3],&signalValues[mySignalStart + ((1 * (1088 + Fr_toInt(&lvar[3]))) + 288)]); // line circom 38
Fr_sub(&expaux[0],&expaux[1],&expaux[2]); // line circom 37
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1)){
KeccakF1600_4_run(mySubcomponents[cmp_index_ref],ctx);

}
}
}else{
{
uint cmp_index_ref = 257;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + ((1 * Fr_toInt(&lvar[3])) + 1600)];
// load src
cmp_index_ref_load = 256;
cmp_index_ref_load = 256;
// end load src
Fr_copy(aux_dest,&ctx->signalValues[ctx->componentMemory[mySubcomponents[256]].signalStart + ((1 * Fr_toInt(&lvar[3])) + 0)]);
}
// run sub component if needed
if(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 1)){
KeccakF1600_4_run(mySubcomponents[cmp_index_ref],ctx);

}
}
}
}
{
PFrElement aux_dest = &lvar[3];
// load src
Fr_add(&expaux[0],&lvar[3],&circuitConstants[2]); // line circom 28
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[3],&circuitConstants[9]); // line circom 28
}
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_add(&expaux[0],&lvar[2],&circuitConstants[2]); // line circom 26
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[4]); // line circom 26
}
{
PFrElement aux_dest = &lvar[2];
// load src
// end load src
Fr_copy(aux_dest,&circuitConstants[1]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[67]); // line circom 45
while(Fr_isTrue(&expaux[0])){
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
Fr_lt(&expaux[0],&lvar[4],&circuitConstants[0]); // line circom 47
while(Fr_isTrue(&expaux[0])){
{
PFrElement aux_dest = &lvar[3];
// load src
cmp_index_ref_load = 257;
cmp_index_ref_load = 257;
Fr_pow(&expaux[2],&circuitConstants[4],&lvar[4]); // line circom 47
Fr_mul(&expaux[1],&ctx->signalValues[ctx->componentMemory[mySubcomponents[257]].signalStart + ((1 * ((8 * Fr_toInt(&lvar[2])) + Fr_toInt(&lvar[4]))) + 0)],&expaux[2]); // line circom 47
Fr_add(&expaux[0],&lvar[3],&expaux[1]); // line circom 47
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
{
PFrElement aux_dest = &lvar[4];
// load src
Fr_add(&expaux[0],&lvar[4],&circuitConstants[2]); // line circom 47
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[4],&circuitConstants[0]); // line circom 47
}
{
PFrElement aux_dest = &signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[2])) + 0)];
// load src
// end load src
Fr_copy(aux_dest,&lvar[3]);
}
{
PFrElement aux_dest = &lvar[2];
// load src
Fr_add(&expaux[0],&lvar[2],&circuitConstants[2]); // line circom 45
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[2],&circuitConstants[67]); // line circom 45
}
for (uint i = 0; i < 258; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0){
assert(!(ctx->componentMemory[index_subc].inputCounter));
release_memory_component(ctx,index_subc);
}
}
}

void Keccak256Hash_6_create(uint soffset,uint coffset,Circom_CalcWit* ctx,std::string componentName,uint componentFather){
ctx->componentMemory[coffset].templateId = 6;
ctx->componentMemory[coffset].templateName = "Keccak256Hash";
ctx->componentMemory[coffset].signalStart = soffset;
ctx->componentMemory[coffset].inputCounter = 288;
ctx->componentMemory[coffset].componentName = componentName;
ctx->componentMemory[coffset].idFather = componentFather;
ctx->componentMemory[coffset].subcomponents = new uint[1]{0};
}

void Keccak256Hash_6_run(uint ctx_index,Circom_CalcWit* ctx){
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
Fr_copy(aux_dest,&circuitConstants[62]);
}
{
std::string new_cmp_name = "keccak";
Keccak_256_bytes_5_create(mySignalStart+320,0+ctx_index+1,ctx,new_cmp_name,myId);
mySubcomponents[0] = 0+ctx_index+1;
}
{
uint cmp_index_ref = 0;
{
PFrElement aux_dest = &ctx->signalValues[ctx->componentMemory[mySubcomponents[cmp_index_ref]].signalStart + 32];
// load src
// end load src
Fr_copyn(aux_dest,&signalValues[mySignalStart + 32],256);
}
// need to run sub component
ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter -= 256;
assert(!(ctx->componentMemory[mySubcomponents[cmp_index_ref]].inputCounter));
Keccak_256_bytes_5_run(mySubcomponents[cmp_index_ref],ctx);
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
Fr_lt(&expaux[0],&lvar[1],&circuitConstants[67]); // line circom 14
while(Fr_isTrue(&expaux[0])){
{
{{
Fr_eq(&expaux[0],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 0)],&signalValues[mySignalStart + ((1 * Fr_toInt(&lvar[1])) + 288)]); // line circom 15
}}
if (!Fr_isTrue(&expaux[0])) std::cout << "Failed assert in template/function " << myTemplateName << " line 15. " <<  "Followed trace of components: " << ctx->getTrace(myId) << std::endl;
assert(Fr_isTrue(&expaux[0]));
}
{
PFrElement aux_dest = &lvar[1];
// load src
Fr_add(&expaux[0],&lvar[1],&circuitConstants[2]); // line circom 14
// end load src
Fr_copy(aux_dest,&expaux[0]);
}
Fr_lt(&expaux[0],&lvar[1],&circuitConstants[67]); // line circom 14
}
for (uint i = 0; i < 1; i++){
uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
if (index_subc != 0){
assert(!(ctx->componentMemory[index_subc].inputCounter));
release_memory_component(ctx,index_subc);
}
}
}

void run(Circom_CalcWit* ctx){
Keccak256Hash_6_create(1,0,ctx,"main",0);
Keccak256Hash_6_run(0,ctx);
}


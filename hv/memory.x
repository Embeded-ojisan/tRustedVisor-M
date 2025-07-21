/* hv/memory.x  */
MEMORY
{
  /* Secure Code @ 0x1000_0000 */
  FLASH (rx)  : ORIGIN = 0x10000000, LENGTH = 512K

  /* Secure SRAM @ 0x3000_0000  （スタック/静的領域用）*/
  RAM   (rwx) : ORIGIN = 0x30000000, LENGTH = 128K
}
REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_DATA", RAM);


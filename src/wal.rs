// Open a file : wal/wal.log
// When a transaction has got all the locks :
//  it should then write to wal.

//  Write in filesystem block size.

//  Write types :
//  a) Transaction A.
//  b) Commit A.

//  b) will only need A's id.

// a) What all should go in A ?
//    Length, { id, vec{key, value} }, padding, [ CRC : at last some bytes ]
//

// Strategy for crash resiliency :

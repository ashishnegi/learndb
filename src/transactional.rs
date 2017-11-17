use consistency::Consistency;

// 1. Try to take all locks that we need from consistency layer.
// 2. If got all locks, then perform the operations in tmp state one by one.
// 3. If any operation failed, rollback i.e. clean up tmp state.
// 4. If all succeeded, commit i.e. move from tmp to real state.

// step 1 : a) We can refactor consistency layer to have a method to
//             take locks without doing the actual work.
//      OR  b) we can have a one-to-one lock mapping in Transactional layer as well.
//
//      b) is simpler and also sounds more robust.
//      a) will require more checks / verification that only one that has taken actual lock
//         is doing the final work.

//      If we go by b), do we need locks on lower layer ?
//
//      Shouldn't Transaction be just a trait and let the Consistency implement it ?
//      So that trait will be write_multiple_keys( [Write(key,value)] )


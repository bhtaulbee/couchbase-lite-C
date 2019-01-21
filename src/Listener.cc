//
// Listener.cc
//
// Copyright © 2019 Couchbase. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#include "Listener.hh"
#include "CBLDatabase.h"


namespace cbl_internal {



}


void CBLListenerToken::remove() {
    assert(_owner);
    _owner->remove(this);
}



#pragma mark - BUFFERING:


void cbl_db_bufferNotifications(CBLDatabase *db,
                                CBLNotificationsReadyCallback callback,
                                void *context)
{
    // TODO
}

/** Immediately issues all pending notifications for this database, by calling their listener
 callbacks. */
void cbl_db_sendNotifications(CBLDatabase *db) {
    // TODO
}
//
//  CBLDatabase_Internal.hh
//  CBL_C
//
//  Created by Jens Alfke on 1/21/19.
//  Copyright © 2019 Couchbase. All rights reserved.
//

#pragma once
#include "CBLDatabase.h"
#include "Internal.hh"
#include "Listener.hh"


struct CBLDatabase : public CBLRefCounted {

    CBLDatabase(C4Database* _cbl_nonnull db,
                const std::string &name_,
                const std::string &path_,
                const std::string &dir_)
    :c4db(db)
    ,name(name_)
    ,path(path_)
    ,dir(dir_)
    { }

    virtual ~CBLDatabase();

    C4Database* const c4db;
    std::string const name;
    std::string const path;
    std::string const dir;

    CBLListenerToken* addListener(CBLDatabaseListener listener _cbl_nonnull, void *context);

private:
    void callListeners();

    cbl_internal::Listeners<CBLDatabaseListener> _listeners;
    C4DatabaseObserver* _observer {nullptr};
};


namespace cbl_internal {
    static inline C4Database* internal(const CBLDatabase *db)    {return db->c4db;}
}
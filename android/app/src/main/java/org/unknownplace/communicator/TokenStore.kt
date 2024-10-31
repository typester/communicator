package org.unknownplace.communicator

import android.content.Context
import uniffi.communicator.TokenStore

const val TOKEN_PREFERENCE_KEY = "org.unknownplace.communicator.token-store"
const val TOKEN_KEY = "token"

class MyTokenStore() : TokenStore {
    override fun get(): String? {
        val pref = SharedContext.context().getSharedPreferences(TOKEN_PREFERENCE_KEY, Context.MODE_PRIVATE)
        return pref.getString(TOKEN_KEY, null)
    }

    override fun set(token: String) {
        val pref = SharedContext.context().getSharedPreferences(TOKEN_PREFERENCE_KEY, Context.MODE_PRIVATE)
        with (pref.edit()) {
            putString(TOKEN_KEY, token)
            apply()
        }
    }
}
package org.unknownplace.communicator

import android.content.Context
import android.net.Uri
import uniffi.communicator.Communicator
import uniffi.communicator.Config
import java.io.File

class Shared private constructor() {
    companion object {
        @Volatile
        private var obj: Communicator? = null

        fun instance() : Communicator {
            if (obj == null) {
                synchronized(this) {
                    if (obj == null) {
                        val databaseFile = File(SharedContext.context().filesDir, "database.db")
                        if (!databaseFile.exists()) {
                            databaseFile.createNewFile()
                        }
                        val databaseUri = Uri.fromFile(databaseFile)

                        val config = Config(
                            tokenStore = MyTokenStore(),
                            databaseUrl = "sqlite://" + databaseUri.path
                        )
                        obj = Communicator(config)
                    }
                }
            }
            return obj!!
        }
    }
}

class SharedContext private constructor() {
    companion object {
        private var context: Context? = null

        fun context(): Context {
            return context!!
        }

        fun setContext(context: Context) {
            this.context = context
        }
    }
}
package org.unknownplace.communicator

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import org.unknownplace.communicator.ui.theme.CommunicatorTheme
import uniffi.communicator.Logger
import uniffi.communicator.initLogger

const val LOGGER_TAG = "Core"

class DebugLogger(): Logger {
    override fun log(msg: String) {
        Log.d(LOGGER_TAG, msg)
    }
}

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()

        initLogger(DebugLogger())
        SharedContext.setContext(applicationContext)

        setContent {
            CommunicatorTheme {
                CommunicatorGraph()
            }
        }
    }
}

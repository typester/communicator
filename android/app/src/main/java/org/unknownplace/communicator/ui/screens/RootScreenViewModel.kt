package org.unknownplace.communicator.ui.screens

import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import org.unknownplace.communicator.Shared
import uniffi.communicator.CommunicatorException

private const val TAG = "RootScreenViewModel"

data class RootScreenUiState(
    val migrating: Boolean = false,
    val finished: Boolean = false,
    val error: String? = null,
)

class RootScreenViewModel: ViewModel() {
    private val _uiState = MutableStateFlow(RootScreenUiState())
    val uiState = _uiState.asStateFlow()

    fun tokenAvailable() : Boolean {
        return Shared.instance().isTokenAvailable()
    }

    fun migrate() {
        viewModelScope.launch {
            val core = Shared.instance()

            try {
                if (core.dbMigrationAvailable()) {
                    _uiState.update { it.copy(migrating = true) }
                    withContext(Dispatchers.IO) {
                        core.dbMigration()
                    }
                    _uiState.update { it.copy(finished = true) }
                }
            } catch (e: CommunicatorException) {
                Log.e(TAG, "migration error", e)
                _uiState.update { it.copy(error = e.toString()) }
            } finally {
                _uiState.update { it.copy(migrating = false) }
            }
        }
    }

    fun resetAndMigrate() {
        viewModelScope.launch {
            _uiState.update { it.copy(error = null) }
            try {
                val core = Shared.instance()
                core.dbReset()
                migrate()
            } catch (e: CommunicatorException) {
                Log.e(TAG, "database reset error", e)
                _uiState.update { it.copy(error = e.toString()) }
            }
        }
    }
}
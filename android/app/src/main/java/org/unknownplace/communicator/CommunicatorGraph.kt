package org.unknownplace.communicator

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import org.unknownplace.communicator.ui.screens.Login
import org.unknownplace.communicator.ui.screens.LoginScreen
import org.unknownplace.communicator.ui.screens.Root
import org.unknownplace.communicator.ui.screens.RootScreen

@Composable
fun CommunicatorGraph() {
    val navController = rememberNavController()

    Column(
        modifier = Modifier.fillMaxSize(),
    ) {
        NavHost(navController, startDestination = Root) {
            composable<Root> {
                RootScreen(
                    navigateToLogin = {
                        navController.navigate(Login) {
                            popUpTo(Root) {
                                inclusive = true
                            }
                        }
                    },
                    navigateToMain = {},
                )
            }

            composable<Login> {
                LoginScreen()
            }
        }
    }
}
package {{ package_name }}

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.togetherWith
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import {{ package_name }}.ui.theme.AndroidRustTheme
import {{ package_name }}.ui.theme.Ink
import {{ package_name }}.ui.theme.Muted
import {{ package_name }}.ui.theme.Paper
import {{ package_name }}.ui.theme.Sand
import {{ package_name }}.ui.theme.SandLight
import {{ package_name }}.ui.theme.SoftMuted

class MainActivity : ComponentActivity() {

    private external fun callRustHome(): String
    private external fun callRustSearch(): String
    private external fun callRustProfile(): String

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        System.loadLibrary("rust_library")

        setContent {
            val items = listOf("Home", "Search", "Profile")
            var selected by remember { mutableStateOf(0) }
            val message = remember(selected) {
                when (selected) {
                    0 -> callRustHome()
                    1 -> callRustSearch()
                    else -> callRustProfile()
                }
            }

            AndroidRustTheme {
                Surface(modifier = Modifier.fillMaxSize()) {
                    Scaffold(
                        bottomBar = {
                            NavigationBar {
                                items.forEachIndexed { index, label ->
                                    val selectedItem = selected == index
                                    NavigationBarItem(
                                        selected = selectedItem,
                                        onClick = { selected = index },
                                        icon = {
                                            Box(
                                                modifier = Modifier
                                                    .size(if (selectedItem) 34.dp else 28.dp)
                                                    .clip(CircleShape)
                                                    .background(
                                                        if (selectedItem) Ink
                                                        else Sand
                                                    ),
                                                contentAlignment = Alignment.Center
                                            ) {
                                                Text(
                                                    text = label.take(1),
                                                    color = if (selectedItem) Paper else Ink,
                                                    fontWeight = FontWeight.Bold
                                                )
                                            }
                                        },
                                        label = { Text(label) }
                                    )
                                }
                            }
                        }
                    ) { padding ->
                        Box(
                            modifier = Modifier
                                .fillMaxSize()
                                .background(
                                    Brush.verticalGradient(
                                        listOf(Paper, SandLight)
                                    )
                                )
                                .padding(padding),
                            contentAlignment = Alignment.Center
                        ) {
                            BottomNavContent(
                                title = items[selected],
                                message = message,
                                highlight = selected
                            )
                        }
                    }
                }
            }
        }
    }
}

@OptIn(ExperimentalAnimationApi::class)
@Composable
private fun BottomNavContent(title: String, message: String, highlight: Int) {
    Column(horizontalAlignment = Alignment.CenterHorizontally) {
        AnimatedContent(
            targetState = title,
            transitionSpec = { fadeIn() togetherWith fadeOut() },
            label = "section"
        ) { state ->
            Text(
                text = state,
                color = Ink,
                style = MaterialTheme.typography.headlineLarge,
                textAlign = TextAlign.Center
            )
        }
        Text(
            text = message,
            color = Muted,
            style = MaterialTheme.typography.bodyLarge,
            textAlign = TextAlign.Center
        )
        Text(
            text = "Selected: ${'$'}{highlight + 1}",
            color = SoftMuted,
            style = MaterialTheme.typography.labelLarge,
            textAlign = TextAlign.Center
        )
    }
}

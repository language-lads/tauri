package com.plugin.microphonelisall

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import androidx.core.content.ContextCompat
import android.Manifest
import android.content.pm.PackageManager
import androidx.core.app.ActivityCompat

@InvokeArg
class PingArgs {
    var value: String? = null
}

@TauriPlugin
class ExamplePlugin(private val activity: Activity) : Plugin(activity) {
    private val implementation = Example()

    @Command
    fun ping(invoke: Invoke) {
        val args = invoke.parseArgs(PingArgs::class.java)

        if (ContextCompat.checkSelfPermission(activity, Manifest.permission.RECORD_AUDIO)
            == PackageManager.PERMISSION_DENIED
        ) {
            ActivityCompat.requestPermissions(
                activity,
                arrayOf(Manifest.permission.RECORD_AUDIO), 1001
            )
        }

        val ret = JSObject()
        ret.put("value", implementation.pong(args.value ?: "default value :("))
        invoke.resolve(ret)
    }
}

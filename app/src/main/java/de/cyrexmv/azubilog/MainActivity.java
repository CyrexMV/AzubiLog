package de.cyrexmv.azubilog;
import android.app.Activity;import android.os.Bundle;import android.webkit.*;import android.print.PrintManager;import android.content.Context;import android.print.PrintAttributes;
public class MainActivity extends Activity {
 private WebView web;
 @Override public void onCreate(Bundle state){super.onCreate(state);getWindow().setStatusBarColor(0xff101827);getWindow().setNavigationBarColor(0xff101827);web=new WebView(this);web.setBackgroundColor(0xff0b1220);WebSettings s=web.getSettings();s.setJavaScriptEnabled(true);s.setDomStorageEnabled(true);s.setAllowFileAccess(true);web.addJavascriptInterface(new Object(){@JavascriptInterface public void printPage(){runOnUiThread(()->{PrintManager pm=(PrintManager)getSystemService(Context.PRINT_SERVICE);pm.print("AzubiLog-Wochenbericht",web.createPrintDocumentAdapter("AzubiLog"),new PrintAttributes.Builder().build());});}},"AzubiLog");web.setWebChromeClient(new WebChromeClient());setContentView(web);web.loadUrl("file:///android_asset/index.html");}
 @Override public void onBackPressed(){if(web.canGoBack())web.goBack();else super.onBackPressed();}
}

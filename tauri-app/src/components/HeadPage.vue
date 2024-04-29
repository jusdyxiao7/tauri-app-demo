<template>
  <div class="custom-page-container">
    <div class="custom-top-header-wrapper">
      <span class="custom-top-header-text">航空搜救任务训练模拟系统</span>
    </div>

    <div class="custom-base-item-container">
      <div class="custom-tool-button-container-wrapper">
        <div class="custom-tool-tip-wrapper">
          <div class="custom-tool-tip-image-wrapper">
            <img
                class="image_logo"
                referrerpolicy="no-referrer"
                src="../assets/student.png"
            />
          </div>
          <div class="custom-tool-button-wrapper">
            <div class="custom-tool-button-image-wrapper" @click="toStudentLogin">
              <img
                  class="image image-margin-top"
                  referrerpolicy="no-referrer"
                  :src="studentCoordinateImageSrc"
                  @mouseover="changeImageActive('toStudentLogin')"
                  @mouseleave="changeImageInactive('toStudentLogin')"
              />
            </div>
            <div class="custom-tool-button-image-wrapper" @click="toStudentOtherLogin">
              <img
                  class="image image-no-top"
                  referrerpolicy="no-referrer"
                  :src="studentCommandImageSrc"
                  @mouseover="changeImageActive('toStudentOtherLogin')"
                  @mouseleave="changeImageInactive('toStudentOtherLogin')"
              />
            </div>
          </div>
        </div>
      </div>
      <div class="custom-tool-button-container-wrapper">
        <div class="custom-tool-tip-wrapper">
          <div class="custom-tool-tip-image-wrapper">
            <img
                class="image_7"
                referrerpolicy="no-referrer"
                src="../assets/teacher.png"
            />
          </div>
          <div class="custom-tool-button-wrapper">
            <div class="custom-tool-button-image-wrapper" @click="toTeacherLogin">
              <img
                  class="image image-margin-top"
                  referrerpolicy="no-referrer"
                  :src="teacherMonitorImageSrc"
                  @mouseover="changeImageActive('toTeacherLogin')"
                  @mouseleave="changeImageInactive('toTeacherLogin')"
              />
            </div>
            <!--
            空的占位
            style="display: block; visibility: hidden;"
            -->
            <div style="display: block; visibility: hidden;" class="custom-tool-button-image-wrapper"
                 @click="openChromeWebSite">
              <img
                  class="image image-no-top"
                  referrerpolicy="no-referrer"
                  :src="teacherMonitorBelowImageSrc"
                  @mouseover="changeImageActive('openChromeWebSite')"
                  @mouseleave="changeImageInactive('openChromeWebSite')"
              />
            </div>
          </div>
        </div>
      </div>
      <div class="custom-tool-button-container-wrapper">
        <div class="custom-tool-tip-wrapper">
          <div class="custom-tool-tip-image-wrapper">
            <img
                class="image_8"
                referrerpolicy="no-referrer"
                src="../assets/extra.png"
            />
          </div>
          <div class="custom-tool-button-wrapper">
            <div class="custom-tool-button-image-wrapper" @click="openLocalAppAudio">
              <img
                  class="image image-margin-top"
                  referrerpolicy="no-referrer"
                  :src="openAppAudioImageSrc"
                  @mouseover="changeImageActive('openLocalAppAudio')"
                  @mouseleave="changeImageInactive('openLocalAppAudio')"
              />
            </div>
            <div class="custom-tool-button-image-wrapper" @click="openLocalApp">
              <img
                  class="image image-no-top"
                  referrerpolicy="no-referrer"
                  :src="openAppInternalCommunicationImageSrc"
                  @mouseover="changeImageActive('openLocalApp')"
                  @mouseleave="changeImageInactive('openLocalApp')"
              />
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="footer">
    </div>
  </div>
</template>
<script>
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {WebviewWindow, appWindow} from "@tauri-apps/api/window";
import { hide, show } from '@tauri-apps/api/app';
import { open, Command } from '@tauri-apps/api/shell';
// 默认图片url
import defaultLogoUrl from '../assets/logo.png';

// 搜救任务协调
import studentCoordinateImageSrc from '../assets/renwuxietiao.png';
import studentCoordinateImageInactiveSrc from '../assets/renwuxietiao.png';
import studentCoordinateImageActiveSrc from '../assets/renwuxietiao-active.png';
// 搜救任务指挥
import studentCommandImageSrc from '../assets/renwuzhihui.png';
import studentCommandImageInactiveSrc from '../assets/renwuzhihui.png';
import studentCommandImageActiveSrc from '../assets/renwuzhihui-active.png';
// 导调监控
import teacherMonitorImageSrc from '../assets/daotiaojiankong.png';
import teacherMonitorImageInactiveSrc from '../assets/daotiaojiankong.png';
import teacherMonitorImageActiveSrc from '../assets/daotiaojiankong-active.png';
// 导调监控-备份
import teacherMonitorBelowImageSrc from '../assets/daotiaojiankongbak.png';
import teacherMonitorBelowImageInactiveSrc from '../assets/daotiaojiankongbak.png';
import teacherMonitorBelowImageActiveSrc from '../assets/daotiaojiankongbak-active.png';
// 语音识别
import openAppAudioImageSrc from '../assets/yuyinshibie.png';
import openAppAudioImageInactiveSrc from '../assets/yuyinshibie.png';
import openAppAudioImageActiveSrc from '../assets/yuyinshibie-active.png';
// 内部通信
import openAppInternalCommunicationImageSrc from '../assets/neibutongxin.png';
import openAppInternalCommunicationImageInactiveSrc from '../assets/neibutongxin.png';
import openAppInternalCommunicationImageActiveSrc from '../assets/neibutongxin-active.png';

// toStudentLoginUrl: 'https://www.baidu.com',
// toStudentOtherLoginUrl: 'https://www.zhihu.com',
// toTeacherLoginUrl: 'https://www.jd.com',
// toAdminLoginUrl: 'https://www.taobao.com',

/**
 * 根据传入的参数创建窗口
 * @param keyCode 唯一编号
 * @param url url 地址
 * @param title 标题
 */
const open_out = (keyCode, url, title) => {
  // 创建一个窗口
  const studentLoginWebView = new WebviewWindow(keyCode, {
    url: url,
    title: title,
    fullscreen: true,
    width: 500,
    height: 400,
    // x: 100,
    // y: 200
  })
  studentLoginWebView.once("tauri://created", () => {
    // alert('搜救任务协调窗口创建')
  })
  studentLoginWebView.once("tauri://error", (error) => {
    // console.log(error);
    // alert(error)
  })
}


export default {
  name: "HeadPage",
  props: {},
  data() {
    return {
      chromeWebSite: "https://www.baidu.com",
      localApp: "notepad.exe",
      localAppAudio: "E:\\Tencent\\WeChat\\WeChat.exe",
      localAppOther: "C:\\Program Files\\DAUM\\PotPlayer\\PotPlayerMini64.exe",

      toStudentLoginUrl: 'http://117.132.0.103:17743/#/loginStudent',
      toStudentOtherLoginUrl: 'http://117.132.0.103:17743/#/loginStudentOther',
      toTeacherLoginUrl: 'http://117.132.0.103:17743/#/loginTeacher',
      toAdminLoginUrl: 'http://117.132.0.103:17743/#/',

      // 默认图片url
      defaultLogoUrl: '../assets/logo.png',

      // 搜救任务协调
      studentCoordinateImageSrc: studentCoordinateImageSrc,
      studentCoordinateImageInactiveSrc: studentCoordinateImageInactiveSrc,
      studentCoordinateImageActiveSrc: studentCoordinateImageActiveSrc,
      // 搜救任务指挥
      studentCommandImageSrc: studentCommandImageSrc,
      studentCommandImageInactiveSrc: studentCommandImageInactiveSrc,
      studentCommandImageActiveSrc: studentCommandImageActiveSrc,
      // 导调监控
      teacherMonitorImageSrc: teacherMonitorImageSrc,
      teacherMonitorImageInactiveSrc: teacherMonitorImageInactiveSrc,
      teacherMonitorImageActiveSrc: teacherMonitorImageActiveSrc,
      // 导调监控-备份
      teacherMonitorBelowImageSrc: teacherMonitorBelowImageSrc,
      teacherMonitorBelowImageInactiveSrc: teacherMonitorBelowImageInactiveSrc,
      teacherMonitorBelowImageActiveSrc: teacherMonitorBelowImageActiveSrc,
      // 语音识别
      openAppAudioImageSrc: openAppAudioImageSrc,
      openAppAudioImageInactiveSrc: openAppAudioImageInactiveSrc,
      openAppAudioImageActiveSrc: openAppAudioImageActiveSrc,
      // 内部通信
      openAppInternalCommunicationImageSrc: openAppInternalCommunicationImageSrc,
      openAppInternalCommunicationImageInactiveSrc: openAppInternalCommunicationImageInactiveSrc,
      openAppInternalCommunicationImageActiveSrc: openAppInternalCommunicationImageActiveSrc,
      constants: {},
    }
  },
  methods: {
    /**
     * 鼠标移入事件
     */
    changeImageActive(key) {
      if (key === 'toStudentLogin') {
        // 搜救任务协调
        this.studentCoordinateImageSrc = this.studentCoordinateImageActiveSrc
      } else if (key === 'toStudentOtherLogin') {
        // 搜救任务指挥
        this.studentCommandImageSrc = this.studentCommandImageActiveSrc
      } else if (key === 'toTeacherLogin') {
        // 导调监控
        this.teacherMonitorImageSrc = this.teacherMonitorImageActiveSrc
      } else if (key === 'openChromeWebSite') {
        // FIXME 预留打开网页
        this.teacherMonitorBelowImageSrc = this.teacherMonitorBelowImageActiveSrc
        // this.openAppAudioImageSrc = this.openAppAudioImageActiveSrc
      } else if (key === 'openLocalApp') {
        // 内部通信
        this.openAppInternalCommunicationImageSrc = this.openAppInternalCommunicationImageActiveSrc
      } else if (key === 'openLocalAppAudio') {
        // 语音识别
        this.openAppAudioImageSrc = this.openAppAudioImageActiveSrc
      } else if (key === 'openLocalAppOther') {
        // FIXME 预留其他App
        // this.openAppInternalCommunicationImageSrc = this.openAppInternalCommunicationImageInactiveSrc
      } else {
        // TODO 其余待扩展

      }
    },
    /**
     * 鼠标离开事件
     */
    changeImageInactive(key) {
      if (key === 'toStudentLogin') {
        this.studentCoordinateImageSrc = this.studentCoordinateImageInactiveSrc
      } else if (key === 'toStudentOtherLogin') {
        // 搜救任务指挥
        this.studentCommandImageSrc = this.studentCommandImageInactiveSrc
      } else if (key === 'toTeacherLogin') {
        // 导调监控
        this.teacherMonitorImageSrc = this.teacherMonitorImageInactiveSrc
      } else if (key === 'openChromeWebSite') {
        // FIXME 预留打开网页
        this.teacherMonitorBelowImageSrc = this.teacherMonitorBelowImageInactiveSrc
        // this.openAppAudioImageSrc = this.openAppAudioImageInactiveSrc
      } else if (key === 'openLocalApp') {
        // 内部通信
        this.openAppInternalCommunicationImageSrc = this.openAppInternalCommunicationImageInactiveSrc
      } else if (key === 'openLocalAppAudio') {
        // 语音识别
        this.openAppAudioImageSrc = this.openAppAudioImageInactiveSrc
      } else if (key === 'openLocalAppOther') {
        // 预留其他本地app
        // this.openAppInternalCommunicationImageSrc = this.openAppInternalCommunicationImageInactiveSrc
      } else {
        // TODO 其余待扩展

      }
    },
    toStudentLogin() {
      open_out('toStudentLogin', this.toStudentLoginUrl, '搜救任务协调')
    },
    toStudentOtherLogin() {
      open_out('toStudentOtherLogin', this.toStudentOtherLoginUrl, '搜救任务指挥')
    },
    toTeacherLogin() {
      open_out('toTeacherLogin', this.toTeacherLoginUrl, '导调监控')
    },
    toAdminLogin() {
      open_out('toAdminLogin', this.toAdminLoginUrl, '导调监控下方预留')
    },
    // 预留打开网页
    openChromeWebSite() {
      open(this.chromeWebSite)
    },
    // 内部通信
    async openLocalApp() {
      invoke('run_script', { scriptPath: 'E:\\replace.bat' });
      // console.log(result)

      // open('notepad.exe')
    },
    // 语音识别
    async openLocalAppAudio() {
      invoke('open_exe', { exePath: 'E:\\Tencent\\WeChat\\WeChat.exe' });
      // console.log(result)

      // try {
      //   // 执行 .exe 文件
      //   await open('E:\\Tencent\\WeChat\\WeChat.exe');
      //   console.log('.exe 文件已成功执行');
      // } catch (error) {
      //   console.error('执行 .exe 文件时出错:', error);
      // }



      // 使用 cmd 运行批处理文件
      // Command.run('E:\\Tencent\\WeChat\\WeChat.exe')
      // const command = new Command('E:\\Tencent\\WeChat\\WeChat.exe');
      // await command.execute();
    },

    // 预留其他app
    openLocalAppOther() {
      // open(this.chromeWebSite)
    }
  }
}
</script>
<style src="./common.css"/>
<style src="./new.css"/>

<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_var _DEBUGGABLE   truevar local  203.2_f6fbbe</name>
   <tag></tag>
   <elementGuidId>f4d6aaf1-6750-4d95-b003-8fc7af66d8cd</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>



 






//var _DEBUGGABLE  = true;
var local = &quot;203.233.63.121&quot;;

$().ready(function() {
	var url = location.href;
	
	if(url.indexOf(&quot;neodev&quot;) > 0) {
		
	}
});

/* //서버쪽 이면 fn_debug를 사용하지 않음 
//if(local == &quot;211.116.54.196&quot; ||  local == &quot;211.116.54.200&quot; || local == &quot;203.233.63.122&quot; || local == &quot;172.17.14.185&quot; ){

if(local.indexOf(&quot;172.17&quot;) > -1 || local.indexOf(&quot;localhost&quot;) > -1 || local.indexOf(&quot;127.0&quot;) > -1){
	_DEBUGGABLE = true;
}else{
	_DEBUGGABLE = false;
}
_DEBUGGABLE = false; */
function home() {
    document.headerForm.action = &quot;/&quot;;
    document.headerForm.submit();
}

function logout() {
    if ( !confirm(&quot;로그아웃하시겠습니까?&quot;) ) {
        return;
    }
    else {
        document.headerForm.action = &quot;/logout&quot;;
        document.headerForm.submit();
    }
}

var WIDTH_LEFT = 150;
var WIDTH_BODY = 862;

var GRID_MINUS_VAL = 45;
var DIV_MINUS_VAL = 50;

var width_before_size=0;


/**
 * 브라우저의 크기가 변할때 메인영역의 크기를 조절함.
 */
window.onresize = function() {
    fq_resize_article();    
	
	var newliLength = $('.newli').length;
	for(var j=1; j&lt;newliLength; j++){   	
		$('#cloneLi'+j).remove();
	}
	
	for(var inx=1; inx&lt;$('.newli').length; inx++){
		$('#hiddenMenu').find(&quot;.newli:eq(&quot;+inx+&quot;)&quot;).attr(&quot;id&quot;,&quot;cloneLi&quot;+inx);	
	}
	
    setMenu();
};


$(window).resize(function (){
	fq_resize_article();    
	
	var newliLength = $('.newli').length;
	for(var j=1; j&lt;newliLength; j++){   	
		$('#cloneLi'+j).remove();
	}
	
	for(var inx=1; inx&lt;$('.newli').length; inx++){
		$('#hiddenMenu').find(&quot;.newli:eq(&quot;+inx+&quot;)&quot;).attr(&quot;id&quot;,&quot;cloneLi&quot;+inx);	
	}
	
    setMenu();
});


var largeMenu;
var lv2Menu;
$(document).ready(function() {
	 $(document).on(&quot;mouseenter&quot;,&quot;.newA&quot;,function(event){
			$(this).addClass(&quot;on&quot;).siblings().removeClass(&quot;on&quot;);
       });
	 $(document).on(&quot;mouseleave&quot;,&quot;.newA&quot;,function(event){		
		 $(this).removeClass(&quot;on&quot;);			
     });
    
	$(&quot;header&quot;).mouseover(function(){
		fn_debug(&quot;header over&quot;);
		$(largeMenu).attr(&quot;src&quot;, function() {
            var src = $(this).attr(&quot;src&quot;);
            return src.replace(&quot;out.gif&quot;,&quot;over.gif&quot;);
        });
        $(lv2Menu).show();
	});

	$(&quot;header&quot;).mouseout(function(){
		fn_debug(&quot;header out&quot;);
		$(largeMenu).attr(&quot;src&quot;, function() {
            var src = $(this).attr(&quot;src&quot;);
            return src.replace(&quot;over.gif&quot;, &quot;out.gif&quot;);
        });
        $(lv2Menu).hide();
 		 
	});

    //메인메뉴 제어
    $(&quot;#mainMenu>li>a&quot;).mouseover(function(){
    	fn_debug(&quot;menu&quot;);
        //해당 a의 부모요소(li).해당 요소의 부모요소를 제외한 부모요소(li)의 형제요소(a, ul).이들 요소 중 ul만.숨겨라
        $(this).parent().siblings().find(&quot;ul&quot;).hide();
        //$(this).next().show(); //하위 메뉴, 즉 레벨2 메뉴 [ul] 객체를 지정(아래 참조)
        menuImg_b = $(this).find(&quot;img&quot;).attr(&quot;src&quot;);
        //alert(menuImg_b);
        $(this).find(&quot;img&quot;).attr(&quot;src&quot;, function() {
            return menuImg_b.replace(&quot;out.gif&quot;, &quot;over.gif&quot;);
        });

        largeMenu = $(this).parent().find(&quot;img&quot;);
        lv2Menu  = $(this).next();
        lst_largeMenu = $(this).parent().find(&quot;img&quot;);
        lst_lv2Menu  = $(this).next();
        $(this).parent().siblings().find(&quot;img&quot;).attr(&quot;src&quot;, function() {
            var src = $(this).attr(&quot;src&quot;);
            return src.replace(&quot;over.gif&quot;, &quot;out.gif&quot;);
        });

        var theV2MenuUl = $(this).next();
        var theV2MenuUlWidth = theV2MenuUl.width();

        var thePosition = theV2MenuUl.position();

        var v1ParentLi = $(this).parent();
        var theV1ParentLiPos = v1ParentLi.position();

        var v1LastLi = $(&quot;#mainMenu > li:last&quot;);
        var theV1LastLiPos = v1LastLi.position();

        var s = &quot;레벨2 현재 UL : left: &quot; + thePosition.left + &quot;, width: &quot; + theV2MenuUl.width() + &quot;\n\n&quot;;
            s+= &quot;레벨1 부모 LI : left: &quot; + theV1ParentLiPos.left + &quot;, width: &quot; + v1LastLi.width() + &quot;\n\n&quot;;
            s+= &quot;레벨1 LAST LI : left: &quot; + theV1LastLiPos.left + &quot;, width: &quot; + v1LastLi.width() + &quot;\n\n&quot;; //alert(s);

        var v2UlEndOffset = theV1ParentLiPos.left + theV2MenuUlWidth;
        var v1LiLastEndOffset = theV1LastLiPos.left + v1LastLi.width();

        if ( v2UlEndOffset > v1LiLastEndOffset ) {
            v2UlEndOffset = v1LiLastEndOffset - theV2MenuUl.width();
            theV2MenuUl.css(&quot;left&quot;, v2UlEndOffset);
        }
        else {
            theV2MenuUl.css(&quot;left&quot;, theV1ParentLiPos.left);
        }
        
        /* 크롬에서 사단 메뉴가 width 가 부족해서 짤리는 현상 수정 +5 처리함*/
        
        if(width_before_size+5 == theV2MenuUlWidth){
        	theV2MenuUl.css(&quot;width&quot;, theV2MenuUlWidth);
        }else{
            width_before_size = theV2MenuUlWidth;
        	theV2MenuUl.css(&quot;width&quot;, theV2MenuUlWidth+5);
        }
        
        
        
        
        theV2MenuUl.show(); //하위 메뉴, 즉 레벨2 메뉴 [ul] 객체를 지정
    })
    .focus(function(){
        $(this).mouseover(); //마우스오버시 실행되는 명령문을 focus할 때 실행
    });
    
    $(&quot;#dv_header&quot;).width(document.documentElement.clientWidth-GRID_MINUS_VAL);
    $(&quot;#dv_body&quot;).width($(&quot;#dv_header&quot;).width()-WIDTH_LEFT);
    $(&quot;.menu1 h3&quot;).on(&quot;click&quot;, function() {
        var level1MenuId = $(this).attr(&quot;rel&quot;);
        var level1MenuName = $(this).text();
        fn_debug(&quot;[default_header] 레벨1 메뉴ID : &quot; + level1MenuId + &quot;, 메뉴명 = '&quot; + level1MenuName + &quot;'&quot;);

        onLevel1MenuSelect(level1MenuId, level1MenuName); //@see default_left.jsp
    }).css('cursor', 'pointer');

    $(&quot;#dialog-system-warn&quot;).dialog({
        autoOpen: false,
        resizable: false,
        height:500,
        width:600,
        modal: true,
        buttons: {
            &quot;확인&quot;: function() {
                //$(&quot;#dialog-system-warn-iframe&quot;).contents().html(&quot;&quot;);
                var iFrame = $(&quot;#dialog-system-warn-iframe&quot;);
                var iFrameDoc = iFrame[0].contentDocument || iFrame[0].contentWindow.document;
                iFrameDoc.write(&quot;&quot;);
                iFrameDoc.close();
                $(this).dialog(&quot;close&quot;);
            }
        }
    });

    $(&quot;#show-msg-dialog&quot;).dialog({
        autoOpen: false,
        resizable: false,
        height:150,
        width:500,
        modal: true,
        buttons: { &quot;확인&quot;: function() { $( this ).dialog( &quot;close&quot; ); } }
    });

    $(&quot;#dialog-pwd-chg&quot;).dialog({
        autoOpen: false,
        resizable: false,
        //height:190,
        //비밀번호 수정항목 적용시 
        height:250,
        width:390,
        modal: true,
        buttons: {
            &quot;저장&quot;: function() {
                changePasswd();
            },
            &quot;닫기&quot;: function() {
                $(&quot;#pwd_old&quot;).val(&quot;&quot;);
                $(&quot;#pwd_new1&quot;).val(&quot;&quot;);
                $(&quot;#pwd_new2&quot;).val(&quot;&quot;);
                $(this).dialog(&quot;close&quot;);
            }
    
        }
    });
    
    $(&quot;.v2menu&quot;).hide();
    
    $(&quot;#btn_sysmenu&quot;).on(&quot;click&quot;, function() {

/*         var hasSysAuth = $(this).data(&quot;has-sys-auth&quot;); //hasFinAuth, hasSysAuth
       
        if (true!=eval(hasSysAuth) &amp;&amp; true!=eval('false')) {
            fq_alert_msg(&quot;관리자 권한이 없습니다.&quot;);
            $(&quot;#btn_sitemap&quot;).focus();
            return false;
           }

        var pgmPath = $(this).attr(&quot;href&quot;);
        if (undefined==pgmPath || ''==pgmPath) {
        	fq_alert_msg(&quot;시스템 메뉴경로를 찾을 수 없습니다!&quot;);
            return false;
        }    */     
		var menuCnt = 0;
    	fq_ajax(&quot;/common/checkAdminAuth&quot;, {DUMMY: &quot;&quot;},
    			function(result) {
    				if (result.success) {
    					
    					menuCnt = parseInt(result.value);
    				}else {
    				    //TODO : 에러시 후처리
    					fq_alert_msg(result.message);
    			    }
    			}
    	);
    	if(menuCnt &lt; 1){	// 가지고 있는 시스템 메뉴가 없으면 들어갈 수 없음
        	fq_alert_msg(&quot;권한이 없습니다.&quot;);
    		return false;
    	}
    	
    });

    $(&quot;#btn_chg_pwd&quot;).on(&quot;click&quot;, function() {

    	var user_id = $(&quot;#pernr&quot;).val();
    	if (user_id.length &lt; 8 || user_id.substr(0,1) == 'D'){
          var pgmPath = $(this).attr(&quot;href&quot;);
          if (undefined==pgmPath || ''==pgmPath || '#'==pgmPath) {
              $(&quot;#dialog-pwd-chg&quot;).dialog(&quot;open&quot;);
              return false;
          }
    	}else{
    		document.form.submit(); 
    		if (opener != null){
    			opener.insert = null;
    			self.close();
    		}
    		
    	}	 
    });
    $(&quot;#btn_sitemap&quot;).on(&quot;click&quot;, function() {
        var pgmPath = $(this).attr(&quot;href&quot;);
        if (undefined==pgmPath || ''==pgmPath || '#'==pgmPath) {
            alert(&quot;작업중...&quot;);
            return false;
        }
    });

    $(&quot;#a_logout&quot;).on(&quot;click&quot;, logout);

    $(&quot;#a_sessinfo&quot;).on(&quot;click&quot;, function() {
        fn_popup_modal(&quot;/common/sessionInfo&quot;, &quot;&quot;, 600, 600, &quot;true&quot;);
    });
    setMenu();
});



function setMenu(){
	   
	var windowWidth = $(window).width();
	var logoIconWidth = $('#logoIcon').css(&quot;width&quot;);
	var topUtilWidth = $('.top_util').css(&quot;width&quot;);		
	
	var newGnbWidth = windowWidth - ((logoIconWidth.replace('px','')*1)+(topUtilWidth.replace('px','')*1) + 40);

	/*if(newGnbWidth > 450){
		$('#gnb').css(&quot;width&quot;,newGnbWidth+'px');   
	}else{		
		newGnbWidth = 450;
		$('#gnb').css(&quot;width&quot;,'450px');   
	}
*/
	
	if(newGnbWidth > 0){
		$('#gnb').css(&quot;width&quot;,newGnbWidth+'px');   
	}
	
    var liTotLength = $('#btn_menu_next').css(&quot;width&quot;).replace('px','')*1 + 10;
    var thLength = $('#gnb').find('.th1').length -1 ;
    
    for(var i=0; i&lt;thLength; i++){
    	liTotLength += $('#gnb').find('.th1:eq('+i+')').css(&quot;width&quot;).replace('px','')*1;
    	
    	if(liTotLength >= newGnbWidth){
    		$('#gnb').find('.th1:eq('+i+')').hide();
    		$('#btn_menu_next').show();

    		/*hidden menu 추가*/
    		var hiddenMenuLength = $('.newli').length;
    		var liClone = $('#cloneLi0').clone();
    		var top = 72;

    		liClone.attr(&quot;id&quot;,&quot;cloneLi&quot;+(hiddenMenuLength));
    		if(hiddenMenuLength == 1){
        		liClone.css(&quot;top&quot;,top+'px');   
    		}else{
        		liClone.css(&quot;top&quot;,top +(35*(hiddenMenuLength-1))+'px');
    		}
    		liClone.find('a').attr(&quot;href&quot;,$('#gnb').find('.th1:eq('+i+')').find('a').attr(&quot;href&quot;));
    		liClone.find('a').text($('#gnb').find('.th1:eq('+i+')').find('a:eq(0)').text());
    		liClone.find('a').bind(&quot;mouseenter mouseleave&quot;);
    		liClone.show();
        	$('#hiddenMenu').append(liClone);
    		
    	}else{
    		$('#gnb').find('.th1:eq('+i+')').show();
    		if(i == thLength-1){
        		$('#btn_menu_next').hide();        		
        		for(var idx=1; idx&lt;$('.newli').length; idx++){
        			$('#hiddenMenu').find('li:eq('+idx+')').remove();
        		}
    		}
    	}
    }
}

function passwdCheck(){
    if(3 != document.getElementById('newpwYn').value){
      alert(&quot;안전하지 않은 비밀번호 입니다.&quot;);

    //document.all.pwd_new1.value='';
    //document.all.pwd_new2.value='';
    $(&quot;#pwd_new1&quot;).val('');
    $(&quot;#pwd_new2&quot;).val('');
    
      return;
    }
    return;
}


//-------------------------------------------------------------------
//패스워드구성 유효성 함수(2015.03.24) 지성
//-------------------------------------------------------------------
function getPasswordLevel(obj) {
    var elNewPasswd = obj;
    var elPasswordLevel = document.getElementById('password_level');
    var result_level = checkPassword.main(elNewPasswd.value);
    var level_color = '';
    var level_txt = '';
    switch(result_level['code']) {
        case 1:
        case 2:
        case 3:
            level_color = '#FF0000';
            level_txt = '적합';
            document.getElementById('newpwYn').value = result_level['code'];
            break;
        case 1000:
        case 2000:
        case 3000:
        case 4000:
            elPasswordLevel.innerHTML = '&lt;font color=&quot;red&quot;>'+result_level['msg']+'&lt;/font>';
            document.getElementById('newpwYn').value = result_level['code'];
            return false;
            break;
        default:
          document.getElementById('newpwYn').value = result_level['code'];
            return false;
            break;
    }
    elPasswordLevel.innerHTML = '보안등급 : &lt;font color=&quot;'+level_color+'&quot; style=&quot;font-weight:bold&quot;>'+level_txt+'&lt;/font>'; 
  
}
var checkPassword = {
    aResultSecure : [],
    sPassword : '',
    sCheckRegexp1 : /^[a-zA-Z]/,
    sCheckRegexp2 : /[a-zA-Z0-9\~\!\@$\^\*\(\)\_\+\{\}\[\]]/,
    sCheckRegexp3 : /^(?=.*[a-zA-Z])(?=.*[^a-zA-Z0-9])(?=.*[0-9]).{8,16}$/,
    sRegexp1 : /[a-z]/,
    sRegexp2 : /[A-Z]/,
    sRegexp3 : /[0-9]/,
    sRegexp4 : /[\~\!\@$\^\*\(\)\_\+\{\}\[\]]/,
    main : function(sPassword) {
        this.aResultSecure['code'] = 0;
        this.aResultSecure['msg'] = false;
        this.sPassword = sPassword;
        //this.sResultRegexp = this.checkRegexp();
        
        // 기본 검사
        if(this.sCheckRegexp3.test(this.sPassword)){
          this.aResultSecure['code'] = 3;
        }else {
          if(this.sPassword.length &lt; 8 || this.sPassword.length > 16) { // 글자수 체크
             this.aResultSecure['code'] = 4000;
             this.aResultSecure['msg'] = '비밀번호를 8자 이상 16자이하 입력해 주세요.';
          }else{
            this.aResultSecure['code'] = 1000;
            this.aResultSecure['msg'] = '영문(대소)+숫자+특수문자 로 조합해 주세요.';
          }
        }
        return this.aResultSecure;
    }

} 



function changePasswd() {
	
    var oldPwd = $.trim( $(&quot;#pwd_old&quot;).val() );
    var newPwd1 = $.trim( $(&quot;#pwd_new1&quot;).val() );
    var newPwd2 = $.trim( $(&quot;#pwd_new2&quot;).val() );

    if (oldPwd.length&lt;1) {
        alert(&quot;기존 비밀번호를 입력하십시오!&quot;);
        return false;
    }
    if (newPwd1.length&lt;1 || newPwd2.length&lt;1) {
        alert(&quot;새로운 비밀번호를 입력하십시오!&quot;);
        return false;
    }
    if (newPwd1 != newPwd2) {
        alert(&quot;신규 비밀번호 재확인을 정확히 입력하십시오!&quot;);
        return false;
    }
    if (newPwd1.length&lt;8) {
        alert(&quot;새로운 비밀번호를 8자리 이상 입력하십시오!&quot;);
        return false;
    }
    if (newPwd1.length>16) {
        alert(&quot;새로운 비밀번호를 16자리 이하 입력하십시오!&quot;);
        return false;
    }
    
    if (oldPwd == newPwd1) {
        alert(&quot;기존비밀번호와 새로운 비밀번로를 다르게 입력하십시오!&quot;);
        return false;
    }
    
    var check = /^(?=.*[a-zA-Z])(?=.*[^a-zA-Z0-9])(?=.*[0-9]).{8,16}$/;
    if(!check.test(newPwd1)){
      alert(&quot;영문(대소문자), 숫자, 특수문자의 조합으로 입력해주세요&quot;);
      return false;
    }
  
    var param = { pwd_old : oldPwd, pwd_new1 : newPwd1, pwd_new2 : newPwd2 };
    var postData = JSON.stringify(param);
    //fn_debug(&quot;[changePasswd] 파라미터 : &quot; + postData);

    $.ajax({
        url         : '/home/chgpwd',   
        type        : 'POST',
        cache       : false,   
        data        : postData,
        contentType : 'application/json; charset=utf-8',   
        traditional : true,
        dataType    : 'json',
        success     : function(result) {
            alert(result.message);
            if ( result.success == true ) {
                fn_debug(&quot;[changePasswd] ajax.success : result.value = &quot; + result.value);
                $(&quot;pwd_old&quot;).val(&quot;&quot;);
                $(&quot;pwd_new1&quot;).val(&quot;&quot;);
                $(&quot;pwd_new2&quot;).val(&quot;&quot;);
                $(&quot;#dialog-pwd-chg&quot;).dialog(&quot;close&quot;);
            }
        },
        error: function(result) {
            fq_ajax_sys_error(result);
        }                    
    });
}






       
                    
                   
            
            	
          			
            			전도금
                  		
                  		
                   			
                      			기준정보
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			신청등록
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			신청승인
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			신청현황
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			실적등록
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			실적현황
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			마감
                   			
                  		
                   			
                  		
                   			
                  		
                  		
              			
          			
        		
          			
            			법인카드
                  		
                  		
                   			
                      			카드관리
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			실적등록
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			한도관리
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			실적현황
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                  		
              			
          			
        		
          			
            			전표입력
                  		
                  		
                   			
                      			원천징수 영수증
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			기타경비
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			SAP전표실적조회
                   			
                  		
                   			
                  		
                   			
                      			계산서발행
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			임차료 및 관리비 전표 처리
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                  		
              			
          			
        		
          			
            			출장경조
                  		
                  		
                   			
                      			출장코드
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			출장신청
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			경조비
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			부임여비
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                  		
              			
          			
        		
          			
            			Mall
                  		
                  		
                   			
                      			판촉비(Mall신청)
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                  		
              			
          			
        		
          			
            			자금배정
                  		
                  		
                   			
                      			기준정보
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			배정등록
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			배정현황
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			배정수정
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			마감
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                  		
              			
          			
        		
          			
            			전표관리
                  		
                  		
                   			
                      			전표 작성/조회
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			세금계산서 관리
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			대손충당금설정
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                  		
              			
          			
        		
          			
            			거래처관리
                  		
                  		
                   			
                      			거래처조회/신청
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                  		
              			
          			
        		
          			
            			전자결재
                  		
                  		
                   			
                      			결재함
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                      			기준정보
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                   			
                  		
                  		
              			
          			
        		
          			
        		
          			
        		
        		
        		
        		>
        			
        				
        			        		
        		
            
                      
        
            IT사업지원팀시스템관리자님
            
            	
                
                	
			             
			          
                
                
                
                
                                   
            
                
    

  

    
    

    
	 
		
		
		
	 





 







    #lmenu li.pgm-selected a { font-weight: bold; color: red; text-decoration: none; }




$(document).ready(function() {
	//좌측 메뉴 닥기/열기 처리
	$(&quot;#img_menu_showhide&quot;).on(&quot;click&quot;, function() {
		var isClosed = $(this).data(&quot;isclosed&quot;);
		//alert(&quot;1- src : &quot; + $(this).attr(&quot;src&quot;) + &quot;\n\n1- isclosed : &quot; + isClosed);

		if ( $(&quot;#leftmenu-aside&quot;).is(':visible') ) {
			
			hideLeftMenu();
		}
		else {
			showLeftMenu();
		}

	});

	$('#lmenu ul').hide();
	$('#lmenu li a').click(function(){
		var checkElement = $(this).next();
		if((checkElement.is('ul')) &amp;&amp; (checkElement.is(':visible'))) {
			$('#lmenu ul:visible').slideUp(300);
			return false;
		}
		if((checkElement.is('ul')) &amp;&amp; (!checkElement.is(':visible'))) {
			$('#lmenu ul:visible').slideUp(300);
			checkElement.slideDown(300);
			return false;
		}
	});

	$(&quot;.pgm-selected&quot;).parent().show();
});

function showLeftMenu() {
	var windowWidth = $(window).width();
	$(&quot;#container aside&quot;).show();
	$(&quot;.leftmenu-button-shdiv&quot;).css(&quot;left&quot;, &quot;206px&quot;);
	$(&quot;#container article&quot;).css(&quot;left&quot;, &quot;0px&quot;);

	if(windowWidth > 1024){
		$(&quot;#container article&quot;).css(&quot;width&quot;, &quot;&quot; + (windowWidth - 230 - 2) +&quot;px&quot;);
	}else{ // 추가 1024이하에서 본분사이즈 수정
		$(&quot;#container article&quot;).css(&quot;width&quot;, &quot;&quot; + (windowWidth - 230 - 2) +&quot;px&quot;);
	}
	
	$(&quot;#img_menu_showhide&quot;).attr(&quot;src&quot;, &quot;/resources/images/common/btn_menuClose.gif&quot;);
	$(&quot;#img_menu_showhide&quot;).data(&quot;isclosed&quot;, &quot;false&quot;);
	fg_resize_by_class();
}
function hideLeftMenu() {
	var windowWidth = $(window).width();
	$(&quot;#container aside&quot;).hide();
	$(&quot;.leftmenu-button-shdiv&quot;).css(&quot;left&quot;, &quot;0px&quot;);
	$(&quot;#container article&quot;).css(&quot;left&quot;, &quot;20px&quot;);
	$(&quot;#container article&quot;).css(&quot;width&quot;, &quot;&quot; + (windowWidth - 20 - 2) +&quot;px&quot;);
	$(&quot;#img_menu_showhide&quot;).attr(&quot;src&quot;, &quot;/resources/images/common/btn_menuOpen.gif&quot;);
	$(&quot;#img_menu_showhide&quot;).data(&quot;isclosed&quot;, &quot;true&quot;);
	fg_resize_by_class();
}



    
      
        

        
        

          
			

        카드관리
          
            카드발급현황
            카드번호관리 
            대행자등록
            카드사번등록/변경
            Project 법인카드 예산신청
          
        
        실적등록
          
            법인카드 사용내역
            법인카드 실적등록(교육국/본부)
            법인카드 실적등록(Staff)
            법인카드 사용내역(관리자)
            법인카드 실적등록 모니터링
            법인카드 감사 모니터링
          
        
        한도관리
          
            한도초기화
            한도조회
            한도조회(관리자용)
            한도변경신청(교육국/본부)
            한도변경신청(Staff)
            한도변경일괄신청
            한도변경확정
          
        
        실적현황
          
            한도대비 사용현황
            한도대비사용현황(관리자용)
            법인카드 사용모니터링
            법인카드사용모니터링(본부)
            신청대비 취소현황
          
        


			
          

        

      

      + window = 1920
+ document = 1920
+ leftmenu = 220
+ article = 1690

    


        
        
        















var cssGreen = {'background-color':GRID_EDITABLE_FALSE_COLOR};
	/*
	 * [자바스크립트 분리 안내] 
	 *
	 * 1. 스크립트가 짧은 경우(100~200라인 이내)이면, 여기(jsp내)에 스크립트 코드를 넣는다.
	 * 2. 그외 스크립트가 길어지는 경우, /resource/js/eacc/[MODULE 2BYTE]/[jsp filename].js 파일로 스크립트를 뺀다.
	 * 3. 필요한 경우 document.ready 함수는 여기(jsp내)에 둘 수 있다.
	 * 4. 그외 예외사항이 있는 경우 '주석'물을 충분히 기재하여 처리한다.
	 */
	 



	
	
		
		
			법인카드 사용내역(관리자)
		
		
		
		
			조회
			
				
					
						
							
								소속
								
									 
									 
									
								
								카드소지자
								
									 
									 
									
								
								청구여부
								
									전체미청구청구  
								
							
							
								사용일자
								
									 ~ 
									
								
								카드번호
								
									
								
								사용구분
								
									전체회사비용개인비용 
								
								
							
						
					
				
			
			
				
			
		
		

		
		
			
				총0건  승인금액:0 신청금액: 0
			

			
				
			
			
			
				 청구여부 결재상태 소속 카드번호 카드소지자 사용일자 사용시간 가맹점명 가맹점업종 가맹점주소 승인번호 승인금액 신청금액 잔여금액 청구월 사용구분 해외사용여부 요청자 결재문서번호 REQ_NO SETMNT_TP 전기일자 
			
		
		
	
	


	
	
	
	
	






시스템 안내close
        
    확인안내close확인비밀번호 변경close
        
            
                
                  ※ 보안등급 높음으로 비밀번호를 설정 하세요. 
                
                
                    기존 비밀번호
                    
                    
                
                
                    신규 비밀번호
                    
                
                
                    신규 비밀번호 재확인
                    
                
                
                  ※ 비밀번호 설정방법 :  8~16자 영문 대 소문자, 숫자, 특수문자를 사용
                
                        
        
    저장닫기소속close




 








var _FIND_LAST_SELECT_ROW;
var _FIND_CURR_SELECT_ROW;

function _find_csks_doInitGrid(params) {
	$(&quot;#_csksfind_grid&quot;).jqGrid({
		url      : &quot;/common/popupGridlist&quot;,
		pager    : &quot;#_csksfind_pager&quot;,
		datatype : &quot;json&quot;,
		type     : &quot;POST&quot;,
		height   : &quot;240&quot;, //세로길이 지정
		width    : &quot;570&quot;,
		rowNum   : 100, //페이징처리 없음
		rowList  : [100,200,300,400,500],
		postData : params,
		viewrecords: true,
		//multiselect: true, // 앞쪽 check box 추가기능 
        jsonReader: {
                root:&quot;gridData&quot;,
                page: &quot;currentPage&quot;,
                total: &quot;totalPages&quot;,
                records: &quot;totalRecords&quot;,
                repeatitems: false             
        },
		colNames:['코스트센터','코스트센터명','책임자'],
		colModel:[
			{name:'KOSTL'		, index:'KOSTL'		, width:200		, align:'left'},  
			{name:'KTEXT'		, index:'KTEXT'		, width:200		, align:'left'},
			{name:'VERAK'		, index:'VERAK'		, width:200		, align:'center'}
		],
		ondblClickRow: function(rowId) {
      		_FIND_CURR_SELECT_ROW = rowId;

      		//다이알로그의 '선택' 버튼을 클릭한 이벤트 트리거링
      		var _u_dialog = $(&quot;#_csksfind_grid_area&quot;).parent();
      			_u_dialog.dialog(&quot;option&quot;, &quot;buttons&quot;)[&quot;선택&quot;].apply(_u_dialog);
      	},
      	onSelectRow: function(rowId) {
      		_FIND_CURR_SELECT_ROW = rowId;
      		_FIND_LAST_SELECT_ROW = rowId;
      	},
      	loadComplete : function(data) {
        	fn_createProgressBar(false);
        }
	});
}

//메인 화며에서 여러개의 공통팝업을 사용할 수 있으므로 '함수명'이 유일해야 함
//따라서 팝업의 각종 함수명들은 그 팝업의 prefix 를 정하여 공통적으로 붙여서 사용
function _find_csks_onSelectRowData() {
  	if (undefined==_FIND_CURR_SELECT_ROW || ''==_FIND_CURR_SELECT_ROW) {
  		alert(&quot;직책을 선택하십시오!&quot;);
  		return null;
  	}

	var rowData = $(&quot;#_csksfind_grid&quot;).jqGrid('getRowData', _FIND_CURR_SELECT_ROW);

	_FIND_CURR_SELECT_ROW = '';
	_FIND_LAST_SELECT_ROW = '';
	$(&quot;#_csksfind_grid&quot;).jqGrid('resetSelection');
	
	return rowData;
}

function _find_csks_doFind() {
	var params = {
			 &quot;FLAG&quot; : &quot;V_CSKS&quot;
			,&quot;SCH_KOSTL&quot; : $.trim($(&quot;#_csksfind_CODE&quot;).val())
			
			//,&quot;SCH_KTEXT&quot; : $.trim($(&quot;#_csksfind_CODE_NM&quot;).val())
	};

	if ( &quot;&quot;==$(&quot;#_csksfind_grid&quot;).html() ) {
		_find_csks_doInitGrid(params); //initialize grid 
	}
	else {
		fn_createProgressBar(true);
		jQuery(&quot;#_csksfind_grid&quot;).jqGrid('setGridParam',{page:1, postData:params}).trigger(&quot;reloadGrid&quot;);
	}
}

var _parent_div_id; //이 다이알로그의 ID값

$(document).ready(function() {
	_parent_div_id = $(&quot;#_csksfind_grid_area&quot;).parent().attr(&quot;id&quot;);

  	$(&quot;#_find_csks_btn_find&quot;).click( function() {
  		_find_csks_doFind();
	});
/* 
  	$(&quot;#_csksfind_CODE&quot;).on(&quot;keypress&quot;, function(e) {
        if(e.keyCode &amp;&amp; e.keyCode == $.ui.keyCode.ENTER) {
        	_find_csks_doFind();  
        }
      });
     */
    $('#findForm').on('submit', function(e) {
     e.preventDefault();
    });
    
	// mskim 
	$(&quot;#_csksfind_grid_area input[type=text]&quot;).keydown(function(e){
  		if(e.keyCode &amp;&amp; e.keyCode == $.ui.keyCode.ENTER) {
  			_find_csks_doFind();  
        }
  	});
    
});






    
        
            
                
	                
	                    코스트센터
	                    
	                        
	                    
	                    
	                
                
            
	    
	
   	
	
		
	
    
	
       
       
    

    



선택닫기/html[1]/body[1]/div[@class=&quot;ui-widget-overlay ui-front&quot;]카드소지자close




 








var _subOwner_FIND_LAST_SELECT_ROW;
var _subOwner_FIND_CURR_SELECT_ROW;

function _find_subOwner_doInitGrid(params) {
	$(&quot;#_sub_ownerfind_grid&quot;).jqGrid({
		url      : &quot;/common/popupGridSubOwnerlist&quot;,
		pager    : &quot;#_subOwnerfind_pager&quot;,
		datatype : &quot;json&quot;,
		type     : &quot;POST&quot;,
		height   : &quot;230&quot;, //세로길이 지정
		width    : &quot;570&quot;,
		rowNum   : 100, //페이징처리 없음
		rowList  : [100,200,300,400,500],
		postData : params,
		viewrecords: true,
		//multiselect: true, // 앞쪽 check box 추가기능 
        jsonReader: {
                root:&quot;gridData&quot;,
                page: &quot;currentPage&quot;,
                total: &quot;totalPages&quot;,
                records: &quot;totalRecords&quot;,
                repeatitems: false             
        },
		colNames:['소속','소속명','사번','성명','직책','직책명'],
		colModel:[
				{name:'KOSTL', 		index:'KOSTL', 		width:120, align:'center'},
				{name:'KTEXT', 		index:'KTEXT', 		width:120, align:'left'},
				{name:'PERNR', 		index:'PERNR', 		width:120, align:'center'},
				{name:'USER_NM', 	index:'USER_NM', 	width:120, align:'left'},
				{name:'DUTY', 		index:'DUTY', 		width:120, align:'left'	,hidden:true},
				{name:'DUTY_NM', 	index:'DUTY_NM', 	width:120, align:'left'	,hidden:true}
		],
		ondblClickRow: function(rowId) {
      		_subOwner_FIND_CURR_SELECT_ROW = rowId;

      		//다이알로그의 '선택' 버튼을 클릭한 이벤트 트리거링
      		var _u_dialog = $(&quot;#_subOwnerfind_grid_area&quot;).parent();
      			_u_dialog.dialog(&quot;option&quot;, &quot;buttons&quot;)[&quot;선택&quot;].apply(_u_dialog);
      	},
      	onSelectRow: function(rowId) {
      		_subOwner_FIND_CURR_SELECT_ROW = rowId;
      		_subOwner_FIND_LAST_SELECT_ROW = rowId;
      	},
      	loadComplete : function(data) {
        	fn_createProgressBar(false);
        }
	});
}

//메인 화며에서 여러개의 공통팝업을 사용할 수 있으므로 '함수명'이 유일해야 함
//따라서 팝업의 각종 함수명들은 그 팝업의 prefix 를 정하여 공통적으로 붙여서 사용
function _find_subOwner_onSelectRowData() {
  	if (undefined==_subOwner_FIND_CURR_SELECT_ROW || ''==_subOwner_FIND_CURR_SELECT_ROW) {
  		return null;
  	}

	var rowData = $(&quot;#_sub_ownerfind_grid&quot;).jqGrid('getRowData', _subOwner_FIND_CURR_SELECT_ROW);

	_subOwner_FIND_CURR_SELECT_ROW = '';
	_subOwner_FIND_LAST_SELECT_ROW = '';
	$(&quot;#_sub_ownerfind_grid&quot;).jqGrid('resetSelection');
	
	return rowData;
}

function _find_subOwner_doFind() {
	var params = {
			 &quot;SCH_PERNR&quot; 	: $.trim($(&quot;#_subOwner_SCH_PERNR&quot;).val())
			,&quot;SCH_USER_NM&quot; 	: $.trim($(&quot;#_subOwner_SCH_USER_NM&quot;).val())
			,&quot;SCH_KOSTL&quot; 	: $.trim($(&quot;#SCH_ORG_KOSTL&quot;).val())
			,&quot;SCH_CARDNO&quot; 	: $.trim($(&quot;#_subOwner_SCH_CARDNO&quot;).val())
			,&quot;SCH_CARD_COR&quot; : $.trim($(&quot;#_subOwner_SCH_CARD_COR&quot;).val())
	};

	if ( &quot;&quot;==$(&quot;#_sub_ownerfind_grid&quot;).html() ) {
		_find_subOwner_doInitGrid(params); //initialize grid 
	}
	else {
		fn_createProgressBar(true);
		jQuery(&quot;#_sub_ownerfind_grid&quot;).jqGrid('setGridParam',{page:1, postData:params}).trigger(&quot;reloadGrid&quot;);
	}
}

var _parent_div_id; //이 다이알로그의 ID값

$(document).ready(function() {
	_parent_div_id = $(&quot;#_subOwnerfind_grid_area&quot;).parent().attr(&quot;id&quot;);

  	$(&quot;#_subOwnerfind_btn_find&quot;).click( function() {
  		_find_subOwner_doFind();
	});

	// mskim 
	$(&quot;#_subOwnerfind_grid_area input[type=text]&quot;).keydown(function(e){
  		if(e.keyCode &amp;&amp; e.keyCode == $.ui.keyCode.ENTER) {
  			_find_subOwner_doFind();  
        }
  	});
  	
});





	
	
		
		
	    
	    	
			
				
			
				
	        
	        	
		            
		            
		                
			                
			                    사번
			                    
			                        
			                    
			                    성명
			                    
			                        
			                    
			                
		                
		                
		            	
		            
	            
	        
	    
	    
		
	       
	       
	    
	
	
	
	


선택닫기</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
   </webElementXpaths>
</WebElementEntity>

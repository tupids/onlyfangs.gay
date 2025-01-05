export interface UpdateApplicationResponse {
  application_id: number;
}

export interface UpdateApplicationRequest {
  status: ApplicationStatus;
}

export enum ApplicationStatus {
  PENDING = 'pending',
  APPROVED = 'approved',
  REJECTED = 'rejected',
  MAYBE = 'maybe',
}

export enum TwitchAccountType {
  PLEB = 'pleb',
  AFFILIATE = 'affiliate',
  PARTNER = 'partner',
}

export interface AddCommentRequest {
  comment: string;
}

export interface AddCommentResponse {
  comment_id: number;
}

export interface Application {
  id: number;
  twitch_id: number;
  twitch_username: string;
  twitch_display_name: string;
  twitch_profile_image_url: string;
  twitch_account_type: TwitchAccountType;
  status: ApplicationStatus;
  reason: string;
  support_clip_url: string;
  follow_count: number;
  created_at: string;
  updated_at: string;
  completed_at: string | null;
}

export interface ApplicationComment {
  id: number;
  application_id: number;
  comment: string;
  twitch_user_id: number;
  twitch_username: string;
  twitch_display_name: string;
  twitch_profile_image_url: string;
  created_at: string;
}
